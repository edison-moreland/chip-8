export function panicHandler() {
    // We expect rust to write panic information to the console
    // our job is just to inform the user that something bad has happend
    window.document.getElementById("errorcard")?.classList.remove("hidden");
}