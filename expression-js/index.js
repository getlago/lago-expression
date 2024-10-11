import { parseExpression } from "./pkg";

var elem = document.getElementById("expression");
var output = document.getElementById("expression-output");
var error = document.getElementById("expression-error");

elem.oninput = function () {
  error.innerHTML = "";
  try {
    output.innerHTML = parseExpression(elem.value);
  } catch (e) {
    error.innerHTML = e;
  }
};
