// 画像を非同期に読み込む関数
async function loadImages() {
    const imageSources = [
        { name: "player", src: "images/player.png" },
        { name: "bullet", src: "images/bullet.png" },
        { name: "enemy", src: "images/enemy.png" },
        { name: "heart", src: "images/heart.png" },
        { name: "background", src: "images/background.png" }
    ];

    const imageObjects = {};

    const promises = imageSources.map(({ name, src }) => {
        return new Promise((resolve, reject) => {
            const image = new Image();
            image.src = src;

            image.onload = () => {
                imageObjects[name] = image;
                resolve();
            };

            image.onerror = (err) => reject(`Failed to load image: ${src}`);
        });
    });

    // すべての画像が読み込まれたら、画像オブジェクトを Rust に渡す
    await Promise.all(promises);

    return imageObjects;  // 画像オブジェクトを返す
}

export { loadImages };
