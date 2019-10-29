const ctx = document.getElementById("screen-canvas").getContext("2d");

export function init(memory) {
  window.renderme = function(x, y, w, h, vec_ptr, ver_len) {
    console.log(vec_ptr, ver_len);
    const ary = new Uint8ClampedArray(memory.buffer, vec_ptr, ver_len);
    const id = new ImageData(ary, w, h);
    ctx.putImageData(id, x, y);
  };
}
