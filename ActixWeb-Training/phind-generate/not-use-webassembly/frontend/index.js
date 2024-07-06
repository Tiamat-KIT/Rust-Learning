// Webカメラにアクセス
navigator.mediaDevices.getUserMedia({ video: true })
  .then(stream => {
    const video = document.querySelector('video');
    video.srcObject = stream;
    video.play();

    // フレーム画像を取得してサーバに送信
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('2d');
    const sendFrame = () => {
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      context.drawImage(video, 0, 0);
      const imageData = canvas.toDataURL('image/png');
      fetch('http://localhost:8080/process-frame', {
        method: 'POST',
        body: imageData
      })
      .then(response => response.json())
      .then(data => {
        const img = document.getElementById('image');
        img.src = `data:image/png;base64,${data.image}`;
      });
      requestAnimationFrame(sendFrame);
    };
    requestAnimationFrame(sendFrame);
  })
  .catch(err => console.error('Error accessing camera:', err));