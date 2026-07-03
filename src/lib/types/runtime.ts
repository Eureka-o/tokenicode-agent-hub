export type RuntimeId = 'claude' | 'codex' | 'gemini' | 'opencode' | 'custom';

export const RUNTIME_LABELS: Record<RuntimeId, string> = {
  claude: 'Claude Code CLI',
  codex: 'Codex CLI',
  gemini: 'Gemini CLI',
  opencode: 'OpenCode',
  custom: 'Custom CLI',
};

export const RUNTIME_ICONS: Record<RuntimeId, string> = {
  claude: '⚡',
  codex: '🧮',
  gemini: '💎',
  opencode: '🔧',
  custom: '⚙️',
};
