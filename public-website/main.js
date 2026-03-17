function setTempLabel(btn, text) {
  const old = btn.textContent;
  btn.textContent = text;
  btn.disabled = true;
  window.setTimeout(() => {
    btn.textContent = old;
    btn.disabled = false;
  }, 900);
}

async function copyText(btn, text) {
  try {
    await navigator.clipboard.writeText(text);
    setTempLabel(btn, "Copied");
  } catch {
    // Fallback for older browsers / insecure contexts
    const ta = document.createElement("textarea");
    ta.value = text;
    ta.setAttribute("readonly", "");
    ta.style.position = "fixed";
    ta.style.top = "-1000px";
    document.body.appendChild(ta);
    ta.select();
    const ok = document.execCommand("copy");
    document.body.removeChild(ta);
    setTempLabel(btn, ok ? "Copied" : "Copy failed");
  }
}

document.addEventListener("click", (e) => {
  const btn = e.target && e.target.closest ? e.target.closest("button.copy") : null;
  if (!btn) return;
  const text = btn.getAttribute("data-copy");
  if (!text) return;
  void copyText(btn, text);
});

