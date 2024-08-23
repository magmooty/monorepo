export const prerender = true;
export const ssr = false;
console.log = (...params) => {
  document.body.innerText += `${JSON.stringify(params)}`;
};
console.error = (...params) => {
  document.body.innerText += `${JSON.stringify(params)}`;
};
console.info = (...params) => {
  document.body.innerText += `${JSON.stringify(params)}`;
};
console.debug = (...params) => {
  document.body.innerText += `${JSON.stringify(params)}`;
};