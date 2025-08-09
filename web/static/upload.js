window.onload = () => {
    let content = document.getElementById("input");

    let decoded = window.atob(content.innerText);
    let blob = new Blob([decoded], { type : 'plain/text' });
    const ds = new DecompressionStream("deflate");
    let stream = blob.stream().pipeThrough(ds);

    content.innerText = stream;
};