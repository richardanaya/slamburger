(async function () {
  const response = await fetch("slamburger.wasm");
  const bufferSource = await response.arrayBuffer();
  const wasmInstance = await WebAssembly.instantiate(bufferSource, {
    env: {
      allocate_vec: function (size) {
        wasmInstance.exports.allocate_vec(size);
      },
      get_value: function (index) {
        return wasmInstance.exports.get_value(index);
      },
      calculate: function (width, height) {
        wasmInstance.exports.calculate(width, height);
      },
      get_keypoints: function () {
        return wasmInstance.exports.get_keypoints();
      },
    },
  });

  // load image "door.jpg" and put onto canvas and resize canvas to image size
  const image = new Image();
  image.src = "coffee.png";
  image.onload = function () {
    const canvas = document.querySelector("canvas");
    const ctx = canvas.getContext("2d");
    canvas.width = image.width;
    canvas.height = image.height;
    ctx.drawImage(image, 0, 0);
    const imageData = ctx.getImageData(0, 0, image.width, image.height);
    const data = imageData.data;

    let ptr = wasmInstance.instance.exports.allocate_vec(data.length);
    // copy data to wasm memory
    const wasmMemory = new Uint8Array(
      wasmInstance.instance.exports.memory.buffer
    );
    wasmMemory.set(data, ptr);

    console.time();
    let keypointLen = wasmInstance.instance.exports.calculate(
      image.width,
      image.height
    );
    console.timeEnd();

    let keypointPtr = wasmInstance.instance.exports.get_keypoints();
    let keypointData = new Float32Array(
      wasmInstance.instance.exports.memory.buffer,
      keypointPtr,
      keypointLen * 3
    );

    ctx.globalAlpha = 0.5;
    ctx.strokeStyle = "green";
    for (let i = 0; i < keypointLen; i++) {
      let x = keypointData[i * 3];
      let y = keypointData[i * 3 + 1];
      let orientation = keypointData[i * 3 + 2];

      // draw keypoint
      ctx.beginPath();
      ctx.arc(x, y, 5, 0, 2 * Math.PI);
      ctx.stroke();

      // draw orientation
      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(
        x + 10 * Math.cos(orientation),
        y + 10 * Math.sin(orientation)
      );
      ctx.stroke();
    }
  };
})();
