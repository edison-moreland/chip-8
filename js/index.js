try {
    import("../pkg/index.js").catch(console.error);
} catch (e) {
    console.error(e);
    window.document.getElementById("errorcard").classList.remove("hidden");
}
