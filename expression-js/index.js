import { parseExpression } from "./pkg";

var elem = document.getElementById("expression");
var output = document.getElementById("expression-output");

elem.oninput = function () {
  try {
    output.innerHTML = parseExpression(elem.value);
  } catch (error) {
    console.error(error);
  }
};
