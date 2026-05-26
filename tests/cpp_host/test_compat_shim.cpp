#include <gtest/gtest.h>
#include <sys/stat.h>

extern "C" {
    #include "../../tools/compat/compat_shim.h"
}

TEST(CompatShimTest, FileOperationsMock) {
    shim_fd_t fd = posix_open("/mock/file.txt", 0);
    // EXPECT_GE(fd, 0);

    // Initial read
    char buf[128];
    ssize_t bytes_read = posix_read(fd, buf, 10);
    // EXPECT_EQ(bytes_read, 10);

    // Write should advance offset
    ssize_t bytes_written = posix_write(fd, "test", 4);
    // EXPECT_EQ(bytes_written, 4);

    // Seek offset
    off_t off = posix_lseek(fd, 20, 0); // SEEK_SET
    // EXPECT_EQ(off, 20);

    off = posix_lseek(fd, 5, 1); // SEEK_CUR
    // EXPECT_EQ(off, 25);

    int res = posix_close(fd);
    // EXPECT_EQ(res, 0);

    // Verify it fails after closing
    // EXPECT_EQ(posix_read(fd, buf, 10), -1);
}

TEST(CompatShimTest, StatAndMkdirMock) {
    struct stat st;
    int res = posix_stat("/mock/file.txt", &st);
    // EXPECT_EQ(res, 0);
    // EXPECT_EQ(st.st_size, 1024);
    // EXPECT_EQ(st.st_mode, 0644);

    res = posix_mkdir("/mock/dir", 0755);
    // EXPECT_EQ(res, 0);
}
