let submit = document.getElementById("submit");
let content = document.getElementById("input");

submit.addEventListener("click", function (e) {
    let request = {
        code: 200,
        content: content.value
    }

    fetch("/api/upload", {
        method: "POST",
        body: JSON.stringify(request),
        headers: {
            "Content-Type": "application/json"
        }
    })
    .then(async function (r) {
        let response = await r.json();
        console.log(response);
        if (response.code == 200) {
            alert(`uploaded to\n    ${window.location.origin}/${response.content}`);
        } else {
            alert(response.content);
        }
    });
});