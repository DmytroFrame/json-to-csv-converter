document.getElementById("input").addEventListener("change", changeInput);
const input = document.getElementById("input");
const output = document.getElementById("output");

async function makeErrorMessage() {
  input.style.backgroundColor = "rgb(255, 144, 144)";
  setInterval(() => {
    input.style.backgroundColor = "";
  }, 2000);
}

async function changeInput() {
  try {
    const body = document.getElementById("input").value;
    JSON.parse(body);
    const requset = await fetch("/api/convert", { method: "POST", body });
    const text = await requset.text();
    output.innerHTML = text;
  } catch {
    makeErrorMessage();
  }
}
