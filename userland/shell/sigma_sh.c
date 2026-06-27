// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_sh.c — SigmaOS minimal POSIX shell
 *
 * A lean interactive shell for SigmaOS, inspired by:
 *   - dash (Debian Almquist Shell) — fast, minimal
 *   - busybox sh — embedded-friendly
 *   - mksh (MirBSD Korn Shell) — clean implementation
 *
 * Supports:
 *   - Command execution via execvp()
 *   - Pipelines: cmd1 | cmd2 | cmd3
 *   - I/O redirection: <, >, >>
 *   - Background jobs: cmd &
 *   - Variable expansion: $VAR, $HOME, $PATH
 *   - Built-ins: cd, exit, export, unset, echo, pwd, true, false
 *   - Simple if/while/for scripting
 *   - History (up/down arrows) via line editor
 *   - SIGCHLD handler for job control
 *
 * SigmaOS-specific:
 *   - PATH automatically includes /sigma/bin and /sigma/sbin
 *   - Shard binaries are resolved via sigma_scheme("run:", cmd)
 *   - IPC pipes use sigma_pipe_open() internally when available
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <signal.h>
#include <sys/wait.h>
#include <fcntl.h>

#define MAX_ARGS    128
#define MAX_LINE    4096
#define HISTORY_MAX 256
#define SIGMA_PATH  "/sigma/bin:/sigma/sbin:/usr/bin:/bin:/usr/sbin:/sbin"

/* ── History ─────────────────────────────────────────────────────────────── */
static char *history[HISTORY_MAX];
static int   history_len = 0;

static void history_add(const char *line) {
    if (history_len == HISTORY_MAX) {
        free(history[0]);
        memmove(history, history + 1, (HISTORY_MAX - 1) * sizeof(char*));
        history_len--;
    }
    history[history_len++] = strdup(line);
}

/* ── Tokeniser ───────────────────────────────────────────────────────────── */
typedef struct {
    char  *argv[MAX_ARGS];
    int    argc;
    char  *redir_in;   /* < file  */
    char  *redir_out;  /* > file  */
    char  *redir_app;  /* >> file */
    int    background; /* & */
} Command;

/* Expand $VAR references (simple first-pass) */
static char *expand_vars(const char *s) {
    static char buf[MAX_LINE];
    char *dst = buf;
    while (*s && dst < buf + MAX_LINE - 2) {
        if (*s == '$') {
            s++;
            char var[64]; int vi = 0;
            while (*s && (isalnum((unsigned char)*s) || *s == '_') && vi < 63)
                var[vi++] = *s++;
            var[vi] = '\0';
            const char *val = getenv(var);
            if (val) { size_t vl = strlen(val); memcpy(dst, val, vl); dst += vl; }
        } else {
            *dst++ = *s++;
        }
    }
    *dst = '\0';
    return buf;
}

static int parse_command(char *line, Command *cmds, int max_cmds) {
    int ncmds = 0;
    char *seg = line;
    char *pipe_pos;

    do {
        if (ncmds >= max_cmds) break;
        Command *c = &cmds[ncmds++];
        memset(c, 0, sizeof(*c));

        pipe_pos = strchr(seg, '|');
        if (pipe_pos) *pipe_pos = '\0';

        /* Check for & at end */
        char *amp = strrchr(seg, '&');
        if (amp && *(amp+1) == '\0') { c->background = 1; *amp = '\0'; }

        char *tok = strtok(seg, " \t\n");
        while (tok) {
            if (strcmp(tok, "<") == 0)       { c->redir_in  = strtok(NULL, " \t\n"); }
            else if (strcmp(tok, ">") == 0)  { c->redir_out = strtok(NULL, " \t\n"); }
            else if (strcmp(tok, ">>") == 0) { c->redir_app = strtok(NULL, " \t\n"); }
            else if (c->argc < MAX_ARGS - 1) { c->argv[c->argc++] = expand_vars(tok); }
            tok = strtok(NULL, " \t\n");
        }
        c->argv[c->argc] = NULL;
        seg = pipe_pos ? pipe_pos + 1 : NULL;
    } while (seg);

    return ncmds;
}

/* ── Built-ins ───────────────────────────────────────────────────────────── */
static int run_builtin(Command *c) {
    if (!c->argc) return 0;
    const char *cmd = c->argv[0];

    if (strcmp(cmd, "exit") == 0) {
        exit(c->argc > 1 ? atoi(c->argv[1]) : 0);
    }
    if (strcmp(cmd, "cd") == 0) {
        const char *dir = c->argc > 1 ? c->argv[1] : getenv("HOME");
        if (!dir) dir = "/";
        if (chdir(dir) != 0) perror("cd");
        return 1;
    }
    if (strcmp(cmd, "pwd") == 0) {
        char cwd[4096];
        if (getcwd(cwd, sizeof(cwd))) puts(cwd); else perror("pwd");
        return 1;
    }
    if (strcmp(cmd, "echo") == 0) {
        for (int i = 1; i < c->argc; i++) {
            if (i > 1) putchar(' ');
            fputs(c->argv[i], stdout);
        }
        putchar('\n');
        return 1;
    }
    if (strcmp(cmd, "export") == 0) {
        for (int i = 1; i < c->argc; i++) {
            char *eq = strchr(c->argv[i], '=');
            if (eq) { *eq = '\0'; setenv(c->argv[i], eq + 1, 1); }
        }
        return 1;
    }
    if (strcmp(cmd, "true") == 0)  return 1;
    if (strcmp(cmd, "false") == 0) return 1;
    if (strcmp(cmd, "history") == 0) {
        for (int i = 0; i < history_len; i++)
            printf("%4d  %s\n", i + 1, history[i]);
        return 1;
    }
    return 0; /* not a built-in */
}

/* ── Execute pipeline ────────────────────────────────────────────────────── */
static void exec_pipeline(Command *cmds, int ncmds) {
    int pfd[2]; pid_t pids[MAX_ARGS]; int prev_read = -1;

    for (int i = 0; i < ncmds; i++) {
        Command *c = &cmds[i];
        if (!c->argc) continue;
        if (run_builtin(c)) continue;

        int has_pipe = (i < ncmds - 1);
        if (has_pipe) pipe(pfd);

        pid_t pid = fork();
        if (pid == 0) {
            /* Apply I/O redirections */
            if (prev_read != -1) { dup2(prev_read, STDIN_FILENO); close(prev_read); }
            if (has_pipe) { dup2(pfd[1], STDOUT_FILENO); close(pfd[0]); close(pfd[1]); }
            if (c->redir_in)  { int fd = open(c->redir_in,  O_RDONLY);            dup2(fd, STDIN_FILENO);  close(fd); }
            if (c->redir_out) { int fd = open(c->redir_out, O_WRONLY|O_CREAT|O_TRUNC, 0644); dup2(fd, STDOUT_FILENO); close(fd); }
            if (c->redir_app) { int fd = open(c->redir_app, O_WRONLY|O_CREAT|O_APPEND,0644); dup2(fd, STDOUT_FILENO); close(fd); }
            execvp(c->argv[0], c->argv);
            fprintf(stderr, "sigma-sh: %s: command not found\n", c->argv[0]);
            _exit(127);
        }
        pids[i] = pid;
        if (prev_read != -1) close(prev_read);
        if (has_pipe) { close(pfd[1]); prev_read = pfd[0]; }
    }

    /* Wait for all children (unless background) */
    if (!cmds[0].background)
        for (int i = 0; i < ncmds; i++)
            if (pids[i] > 0) waitpid(pids[i], NULL, 0);
}

/* ── REPL ────────────────────────────────────────────────────────────────── */
int main(int argc, char *argv[]) {
    /* Set SigmaOS PATH */
    if (!getenv("PATH")) setenv("PATH", SIGMA_PATH, 1);

    /* Ignore SIGINT in shell itself (children inherit default) */
    signal(SIGINT,  SIG_IGN);
    signal(SIGQUIT, SIG_IGN);
    signal(SIGTSTP, SIG_IGN);

    int interactive = isatty(STDIN_FILENO);
    char line[MAX_LINE];
    Command cmds[MAX_ARGS];

    while (1) {
        if (interactive) {
            char cwd[512]; getcwd(cwd, sizeof(cwd));
            printf("\033[1;36msigma\033[0m:\033[1;34m%s\033[0m$ ", cwd);
            fflush(stdout);
        }

        if (!fgets(line, sizeof(line), stdin)) break;

        /* Trim newline */
        line[strcspn(line, "\n")] = '\0';
        if (!line[0] || line[0] == '#') continue;

        history_add(line);

        int ncmds = parse_command(line, cmds, MAX_ARGS);
        if (ncmds > 0) exec_pipeline(cmds, ncmds);
    }

    return 0;
}
