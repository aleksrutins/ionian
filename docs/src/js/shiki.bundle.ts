/* Generate by @shikijs/codegen */
import type {
  DynamicImportLanguageRegistration,
  DynamicImportThemeRegistration,
  HighlighterGeneric,
} from '@shikijs/types'
import {
  createSingletonShorthands,
  createdBundledHighlighter,
} from '@shikijs/core'
import { createJavaScriptRegexEngine } from '@shikijs/engine-javascript'

type BundledLanguage =
  | 'jinja'
  | 'toml'
  | 'shellscript'
  | 'bash'
  | 'sh'
  | 'shell'
  | 'zsh'
  | 'html'
type BundledTheme = 'rose-pine' | 'rose-pine-dawn'
type Highlighter = HighlighterGeneric<BundledLanguage, BundledTheme>

const bundledLanguages = {
  jinja: () => import('@shikijs/langs/jinja'),
  toml: () => import('@shikijs/langs/toml'),
  shellscript: () => import('@shikijs/langs/shellscript'),
  bash: () => import('@shikijs/langs/shellscript'),
  sh: () => import('@shikijs/langs/shellscript'),
  shell: () => import('@shikijs/langs/shellscript'),
  zsh: () => import('@shikijs/langs/shellscript'),
  html: () => import('@shikijs/langs/html'),
} as Record<BundledLanguage, DynamicImportLanguageRegistration>

const bundledThemes = {
  'rose-pine': () => import('@shikijs/themes/rose-pine'),
  'rose-pine-dawn': () => import('@shikijs/themes/rose-pine-dawn'),
} as Record<BundledTheme, DynamicImportThemeRegistration>

const createHighlighter = /* @__PURE__ */ createdBundledHighlighter<
  BundledLanguage,
  BundledTheme
>({
  langs: bundledLanguages,
  themes: bundledThemes,
  engine: () => createJavaScriptRegexEngine(),
})

const {
  codeToHtml,
  codeToHast,
  codeToTokensBase,
  codeToTokens,
  codeToTokensWithThemes,
  getSingletonHighlighter,
  getLastGrammarState,
} = /* @__PURE__ */ createSingletonShorthands<BundledLanguage, BundledTheme>(
  createHighlighter,
)

export {
  bundledLanguages,
  bundledThemes,
  codeToHast,
  codeToHtml,
  codeToTokens,
  codeToTokensBase,
  codeToTokensWithThemes,
  createHighlighter,
  getLastGrammarState,
  getSingletonHighlighter,
}
export type { BundledLanguage, BundledTheme, Highlighter }
