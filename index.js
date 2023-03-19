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

  // watch webcamera and put onto canvas
  const video = document.querySelector("video");
  const canvas = document.querySelector("canvas");
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  const constraints = {
    video: {
      width: 640,
      height: 480,
    },
  };
  const stream = await navigator.mediaDevices.getUserMedia(constraints);
  video.srcObject = stream;
  video.onloadedmetadata = function (e) {
    video.play();
  };

  const frame_1 = document.createElement("canvas");
  const frame_2 = document.createElement("canvas");
  let frame_1_ctx = undefined;
  let frame_2_ctx = undefined;

  video.onplay = function () {
    const run = function () {
      // get video size
      const video = document.querySelector("video");

      const width = video.videoWidth;
      const height = video.videoHeight;

      if (width === 0 || height === 0) {
        return;
      }

      if (frame_1_ctx === undefined && frame_2_ctx === undefined) {
        frame_1.width = width;
        frame_1.height = height;
        frame_1_ctx = frame_1.getContext("2d");
        frame_1_ctx.drawImage(video, 0, 0);
        canvas.width = width * 2;
        canvas.height = height;
      } else if (frame_1_ctx !== undefined && frame_2_ctx === undefined) {
        frame_2.width = width;
        frame_2.height = height;
        frame_2_ctx = frame_2.getContext("2d");
        frame_2_ctx.drawImage(frame_1, 0, 0);
        frame_1_ctx.drawImage(video, 0, 0);
      } else if (frame_1_ctx !== undefined && frame_2_ctx !== undefined) {
        frame_2_ctx.drawImage(frame_1, 0, 0);
        frame_1_ctx.drawImage(video, 0, 0);
      }

      ctx.drawImage(frame_1, 0, 0);
      ctx.drawImage(frame_2, width, 0);

      /*const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      const data = imageData.data;

      let ptr = wasmInstance.instance.exports.allocate_vec(data.length);
      // copy data to wasm memory
      const wasmMemory = new Uint8Array(
        wasmInstance.instance.exports.memory.buffer
      );
      wasmMemory.set(data, ptr);

      console.time();
      let keypointLen = wasmInstance.instance.exports.calculate(
        canvas.width,
        canvas.height
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
      }*/
      requestAnimationFrame(run);
    };
    requestAnimationFrame(run);
  };
})();
