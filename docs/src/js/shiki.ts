import { codeToHtml } from "./shiki.bundle";

document.querySelectorAll("pre:has(>code)").forEach(async (el) => {
  let lang =
    Array.from(el?.querySelector("&>code")?.classList ?? [])
      .find((c) => c.startsWith("language-"))
      ?.substring("language-".length) ?? "html";
  el.outerHTML = await codeToHtml(el.textContent ?? "", {
    lang,
    theme: "rose-pine",
  });
});
