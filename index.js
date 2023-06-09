let total_frames = 0;
let average_fps = 0;
let last_time = 0;

// function to keep average fps
function update_fps() {
  total_frames++;
  const now = performance.now();
  const delta = now - last_time;
  if (delta > 1000) {
    average_fps = total_frames / (delta / 1000);
    total_frames = 0;
    last_time = now;
  }
}

(async function () {
  const response = await fetch("slamburger.wasm");
  const bufferSource = await response.arrayBuffer();
  const wasmInstance = await WebAssembly.instantiate(bufferSource, {
    env: {
      get_value: function (index) {
        return wasmInstance.exports.get_value(index);
      },
      calculate: function (width, height) {
        wasmInstance.exports.calculate(width, height);
      },
      get_keypoints: function () {
        return wasmInstance.exports.get_keypoints();
      },
      js_log: function (signal) {
        console.log(signal);
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
  let frame1_ptr = undefined;
  let frame2_ptr = undefined;

  let slot = 0;

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

      if (slot === 0) {
        const imageData = ctx.getImageData(0, 0, frame_1.width, frame_1.height);
        const data = imageData.data;
        if (frame1_ptr === undefined) {
          frame1_ptr = wasmInstance.instance.exports.allocate_slot_0(
            data.length
          );
        }
        const wasmMemory = new Uint8Array(
          wasmInstance.instance.exports.memory.buffer
        );
        wasmMemory.set(data, frame1_ptr);
        slot = 1;
      } else {
        const imageData = ctx.getImageData(0, 0, frame_2.width, frame_2.height);
        const data = imageData.data;
        if (frame2_ptr === undefined) {
          frame2_ptr = wasmInstance.instance.exports.allocate_slot_1(
            data.length
          );
        }
        const wasmMemory = new Uint8Array(
          wasmInstance.instance.exports.memory.buffer
        );
        wasmMemory.set(data, frame2_ptr);
        slot = 0;
      }

      if (frame1_ptr !== undefined && frame2_ptr !== undefined) {
        const result = wasmInstance.instance.exports.calculate(
          frame_1.width,
          frame_1.height,
          slot
        );

        console.log(result);

        let greyPtr = wasmInstance.instance.exports.get_grayscale();
        let greyLen = wasmInstance.instance.exports.get_grayscale_len();

        let greyData = new Uint8Array(
          wasmInstance.instance.exports.memory.buffer,
          greyPtr,
          greyLen
        );

        let greyImageData = new ImageData(
          new Uint8ClampedArray(greyData),
          frame_1.width,
          frame_1.height
        );

        ctx.putImageData(greyImageData, 0, 0);

        let keypointPtr0 = wasmInstance.instance.exports.get_keypoints_slot_0();
        let keypointsLen0 =
          wasmInstance.instance.exports.get_keypoints_slot_0_len();
        let keypointData = new Float32Array(
          wasmInstance.instance.exports.memory.buffer,
          keypointPtr0,
          keypointsLen0 * 3
        );

        ctx.globalAlpha = 0.5;
        ctx.strokeStyle = "green";
        for (let i = 0; i < keypointsLen0; i++) {
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

        // now draw the second slot
        let keypointPtr1 = wasmInstance.instance.exports.get_keypoints_slot_1();
        let keypointsLen1 =
          wasmInstance.instance.exports.get_keypoints_slot_1_len();
        keypointData = new Float32Array(
          wasmInstance.instance.exports.memory.buffer,
          keypointPtr1,
          keypointsLen1 * 3
        );

        ctx.globalAlpha = 0.5;
        ctx.strokeStyle = "green";
        for (let i = 0; i < keypointsLen1; i++) {
          let x = keypointData[i * 3] + width;
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
      }

      let keypointMatchesPtr =
        wasmInstance.instance.exports.get_keypoints_matches();
      let keypointMatchesLen =
        wasmInstance.instance.exports.get_keypoints_matches_len();
      let keypointData = new Float32Array(
        wasmInstance.instance.exports.memory.buffer,
        keypointMatchesPtr,
        keypointMatchesLen * 3
      );

      //for each pair, draw a line
      ctx.globalAlpha = 1;
      ctx.strokeStyle = "red";
      for (let i = 0; i < keypointMatchesLen; i += 2) {
        let x0 = keypointData[i * 3];
        let y0 = keypointData[i * 3 + 1];

        let x1 = keypointData[i * 3 + 3] + width;
        let y1 = keypointData[i * 3 + 4];

        ctx.beginPath();
        ctx.moveTo(x0, y0);
        ctx.lineTo(x1, y1);
        ctx.stroke();
      }

      // draw fps
      ctx.globalAlpha = 1;
      ctx.fillStyle = "white";
      ctx.font = "20px Arial";
      ctx.fillText("FPS: " + average_fps.toFixed(0), 10, 30);

      update_fps();
      requestAnimationFrame(run);
    };
    requestAnimationFrame(run);
  };
})();
