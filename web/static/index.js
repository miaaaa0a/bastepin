let submit = document.getElementById("submit");
let content = document.getElementById("input");

// https://stackoverflow.com/a/30810322
function fallbackCopyTextToClipboard(text) {
  var textArea = document.createElement("textarea");
  textArea.value = text;
  
  // Avoid scrolling to bottom
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.position = "fixed";

  document.body.appendChild(textArea);
  textArea.focus();
  textArea.select();
  var successful = false;

  try {
    var successful = document.execCommand('copy');
  } catch (err) {
    console.error('Fallback: Oops, unable to copy', err);
  }

  document.body.removeChild(textArea);
  return successful;
}

function copyTextToClipboard(text) {
  if (!navigator.clipboard) {
    let success = fallbackCopyTextToClipboard(text);
    return success;
  }
  navigator.clipboard.writeText(text).then(function() {
    return true;
  }, function(err) {
    console.error('Async: Could not copy text: ', err);
    return false;
  });
}

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
            let upload_url = `${window.location.origin}/${response.content}`;
            let success = copyTextToClipboard(upload_url);
            let clipboard_info = success ? 'was not' : 'was';
            alert(`uploaded to\n    ${upload_url}\n( url ${clipboard_info} copied to your clipboard ^_^ )`);
            
        } else {
            alert(response.content);
        }
    });
});