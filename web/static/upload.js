window.onload = async () => {
    let content = document.getElementById("input");

    let decoded = Uint8Array.from(atob(content.innerHTML), c => c.charCodeAt(0));
    let blob = new Blob([decoded], { type : 'application/octet-stream' });
    const ds = new DecompressionStream("deflate");
    let stream = blob.stream().pipeThrough(ds);

    const response = new Response(stream);
    let text = await response.text();
    content.innerHTML = text.replaceAll("\n", "&#13;&#10;")
};