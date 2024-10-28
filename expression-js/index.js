import { parseExpression, evaluateExpression } from "./pkg";

var elem = document.getElementById("expression");
var output = document.getElementById("expression-output");
var error = document.getElementById("expression-error");

var expression = null;

elem.oninput = function () {
  error.innerHTML = "";
  try {
    expression = parseExpression(elem.value);
    output.innerHTML = evaluateExpression(expression, {});
  } catch (e) {
    error.innerHTML = e;
  }
};
