// SPDX-License-Identifier: GPL-2.0-or-later
// CodeExplorer.tsx — Interactive code browser for SigmaOS implementation files
// Template: bolt-vite-react-ts

import { useState, useMemo } from 'react';
import {
  Cpu, HardDrive, Folder, Network, Monitor, Lock,
  Zap, Terminal, Music, Layers, Battery,
  ChevronRight, ChevronDown, FileCode, Copy, Check,
  Search, X, Code, FolderOpen,
} from 'lucide-react';
import { codeCategories, type CodeFile } from '../data/codeExamples';

const iconMap: Record<string, React.ElementType> = {
  Cpu, HardDrive, Folder, Network, Monitor, Lock,
  Zap, Terminal, Music, Layers, Battery,
};

// ── Syntax highlighter ────────────────────────────────────────────────────────
function SyntaxHighlighter({ code }: { code: string }) {
  const lines = useMemo(() => {
    const keywords = new Set([
      'namespace','class','struct','public','private','protected',
      'virtual','override','static','const','constexpr','return',
      'if','else','for','while','switch','case','break','continue',
      'new','delete','sizeof','typedef','using','template','typename',
      'enum','true','false','nullptr','void','int','auto',
      'inline','explicit','noexcept','operator','this',
    ]);
    const types = new Set([
      'uint8_t','uint16_t','uint32_t','uint64_t','int8_t','int16_t',
      'int32_t','int64_t','uintptr_t','size_t','ssize_t','bool',
      'float','double','char','unsigned','signed',
    ]);
    const macros = new Set([
      '#define','#ifdef','#ifndef','#endif','#include','#pragma',
      '#if','#else','#elif','#undef',
    ]);

    return code.split('\n').map((line, i) => {
      const isComment = line.trim().startsWith('//') || line.trim().startsWith('*') || line.trim().startsWith('/*');
      if (isComment) {
        return <span key={i} className="block text-emerald-600">{line}</span>;
      }

      const tokenRegex = /("(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\b[a-zA-Z_]\w*\b|\b0x[\da-fA-F]+\b|\b\d+\b|#\w+|[{}()\[\];,.]|->|::|\+\+|--|<<|>>|==|!=|<=|>=|&&|\|\||\S|\s+)/g;
      const tokens: { type: string; text: string }[] = [];
      let m: RegExpExecArray | null;
      while ((m = tokenRegex.exec(line)) !== null) {
        const t = m[0];
        let type = 'text';
        if (t.startsWith('"') || t.startsWith("'")) type = 'string';
        else if (macros.has(t)) type = 'macro';
        else if (keywords.has(t)) type = 'keyword';
        else if (types.has(t)) type = 'type';
        else if (/^(0x[\da-fA-F]+|\d+)$/.test(t)) type = 'number';
        else if (t === '->' || t === '::') type = 'operator';
        else if ('{}()[]'.includes(t)) type = 'bracket';
        else if (';,=+-%&|!~^<>*/?:'.includes(t)) type = 'operator';
        tokens.push({ type, text: t });
      }

      const colorClass: Record<string, string> = {
        keyword:  'text-amber-600 font-semibold',
        type:     'text-cyan-600',
        string:   'text-emerald-600',
        number:   'text-rose-500',
        macro:    'text-red-600 font-semibold',
        operator: 'text-slate-500',
        bracket:  'text-slate-400',
        text:     'text-slate-800 dark:text-slate-200',
      };

      return (
        <span key={i} className="block">
          {tokens.map((tok, j) => (
            <span key={j} className={colorClass[tok.type] || 'text-slate-800'}>{tok.text}</span>
          ))}
        </span>
      );
    });
  }, [code]);

  return <pre className="font-mono text-xs leading-relaxed overflow-x-auto">{lines}</pre>;
}

// ── Main component ────────────────────────────────────────────────────────────
export default function CodeExplorer() {
  const [selectedFile, setSelectedFile] = useState<CodeFile | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [copied, setCopied] = useState(false);
  const [expanded, setExpanded] = useState<Set<string>>(new Set(['drivers', 'storage']));

  const allFiles = useMemo(() => codeCategories.flatMap(c => c.files), []);

  const filteredFiles = useMemo(() => {
    if (!searchTerm) return null;
    const t = searchTerm.toLowerCase();
    return allFiles.filter(f =>
      f.filename.toLowerCase().includes(t) ||
      f.description.toLowerCase().includes(t) ||
      f.code.toLowerCase().includes(t)
    );
  }, [allFiles, searchTerm]);

  const handleCopy = async () => {
    if (!selectedFile) return;
    await navigator.clipboard.writeText(selectedFile.code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const toggleCat = (id: string) => {
    const next = new Set(expanded);
    next.has(id) ? next.delete(id) : next.add(id);
    setExpanded(next);
  };

  const totalFiles = allFiles.length;

  return (
    <div className="min-h-screen bg-slate-50">
      <div className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">

        {/* Header */}
        <div className="mb-6">
          <div className="flex items-center gap-3 mb-1">
            <div className="p-2 bg-slate-900 rounded-lg">
              <Code className="w-5 h-5 text-white" />
            </div>
            <h1 className="text-2xl font-bold text-slate-900">SigmaOS Code Explorer</h1>
          </div>
          <p className="text-sm text-slate-500">
            Browse {totalFiles} implementation files across {codeCategories.length} subsystems.
            Syntax-highlighted C++, Go, Assembly, and Shell.
          </p>
        </div>

        {/* Search */}
        <div className="mb-4 relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
          <input
            type="text"
            placeholder="Search files, descriptions, code content..."
            value={searchTerm}
            onChange={e => setSearchTerm(e.target.value)}
            className="w-full pl-10 pr-8 py-2.5 border border-slate-200 rounded-xl text-sm
                       bg-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          {searchTerm && (
            <button
              onClick={() => setSearchTerm('')}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600"
            >
              <X className="w-4 h-4" />
            </button>
          )}
        </div>

        {/* Layout */}
        <div className="flex gap-5">

          {/* Sidebar */}
          <div className="w-72 shrink-0">
            <div className="bg-white rounded-xl border border-slate-200 shadow-sm overflow-hidden">
              <div className="px-4 py-3 border-b border-slate-100 bg-slate-50 flex items-center gap-2">
                <FolderOpen className="w-4 h-4 text-slate-500" />
                <span className="text-sm font-semibold text-slate-700">File Tree</span>
                <span className="ml-auto text-xs text-slate-400">{totalFiles} files</span>
              </div>
              <div className="max-h-[72vh] overflow-y-auto">

                {/* Search results */}
                {filteredFiles ? (
                  <div className="p-2">
                    {filteredFiles.length === 0 ? (
                      <p className="text-xs text-slate-400 text-center py-4">No matches</p>
                    ) : filteredFiles.map(f => (
                      <button
                        key={f.id}
                        onClick={() => setSelectedFile(f)}
                        className={`w-full text-left px-3 py-2 rounded-lg text-xs flex items-center gap-2
                          ${selectedFile?.id === f.id ? 'bg-blue-50 text-blue-700' : 'hover:bg-slate-50 text-slate-700'}`}
                      >
                        <FileCode className="w-3.5 h-3.5 text-slate-400 shrink-0" />
                        <span className="truncate">{f.filename.split('/').pop()}</span>
                      </button>
                    ))}
                  </div>
                ) : (
                  /* Category tree */
                  codeCategories.map(cat => {
                    const Icon = iconMap[cat.icon] || Code;
                    const isOpen = expanded.has(cat.id);
                    return (
                      <div key={cat.id}>
                        <button
                          onClick={() => toggleCat(cat.id)}
                          className="w-full flex items-center gap-2 px-4 py-2.5 hover:bg-slate-50 text-sm"
                        >
                          {isOpen
                            ? <ChevronDown className="w-3.5 h-3.5 text-slate-400 shrink-0" />
                            : <ChevronRight className="w-3.5 h-3.5 text-slate-400 shrink-0" />}
                          <Icon className="w-4 h-4 text-slate-500 shrink-0" />
                          <span className="font-medium text-slate-700">{cat.name}</span>
                          <span className="ml-auto text-xs text-slate-400">{cat.files.length}</span>
                        </button>
                        {isOpen && (
                          <div className="border-l-2 border-slate-100 ml-7">
                            {cat.files.map(f => (
                              <button
                                key={f.id}
                                onClick={() => setSelectedFile(f)}
                                className={`w-full text-left px-3 py-1.5 text-xs flex items-center gap-2
                                  ${selectedFile?.id === f.id
                                    ? 'bg-blue-50 text-blue-700'
                                    : 'text-slate-600 hover:bg-slate-50'}`}
                              >
                                <FileCode className="w-3 h-3 text-slate-400 shrink-0" />
                                <span className="truncate">{f.filename.split('/').pop()}</span>
                              </button>
                            ))}
                          </div>
                        )}
                      </div>
                    );
                  })
                )}
              </div>
            </div>
          </div>

          {/* Code viewer */}
          <div className="flex-1 min-w-0">
            {selectedFile ? (
              <div className="bg-white rounded-xl border border-slate-200 shadow-sm overflow-hidden">
                {/* File header */}
                <div className="flex items-center justify-between px-5 py-3 bg-slate-50 border-b border-slate-100">
                  <div>
                    <div className="flex items-center gap-2">
                      <FileCode className="w-4 h-4 text-slate-500" />
                      <span className="font-mono text-sm font-semibold text-slate-800">
                        {selectedFile.filename}
                      </span>
                    </div>
                    <p className="text-xs text-slate-400 mt-0.5">{selectedFile.description}</p>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-xs px-2 py-1 bg-slate-200 rounded font-mono text-slate-600">
                      {selectedFile.language}
                    </span>
                    <button
                      onClick={handleCopy}
                      className="p-2 hover:bg-slate-200 rounded-lg transition-colors text-slate-500"
                      title="Copy to clipboard"
                    >
                      {copied
                        ? <Check className="w-4 h-4 text-emerald-600" />
                        : <Copy className="w-4 h-4" />}
                    </button>
                  </div>
                </div>
                {/* Code */}
                <div className="p-5 overflow-auto max-h-[68vh] bg-white">
                  <SyntaxHighlighter code={selectedFile.code} />
                </div>
              </div>
            ) : (
              <div className="bg-white rounded-xl border border-slate-200 shadow-sm p-16 text-center">
                <FileCode className="w-14 h-14 text-slate-200 mx-auto mb-4" />
                <h3 className="text-lg font-semibold text-slate-600 mb-2">Select a file to view</h3>
                <p className="text-sm text-slate-400 max-w-sm mx-auto">
                  Choose any file from the sidebar to explore SigmaOS implementation code.
                </p>
                <div className="mt-6 flex flex-wrap gap-2 justify-center">
                  {codeCategories.slice(0, 6).map(cat => {
                    const Icon = iconMap[cat.icon] || Code;
                    return (
                      <button
                        key={cat.id}
                        onClick={() => {
                          setExpanded(prev => new Set([...prev, cat.id]));
                          setSelectedFile(cat.files[0]);
                        }}
                        className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-50 hover:bg-slate-100
                                   border border-slate-200 rounded-lg text-xs text-slate-600"
                      >
                        <Icon className="w-3 h-3" />
                        {cat.name}
                      </button>
                    );
                  })}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
