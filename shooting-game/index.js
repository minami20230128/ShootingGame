import { run, Player, Bullet } from './pkg/shooting_game.js';  // 名前付きインポート

async function runGame() {
    await run(); // wasmの初期化
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');

    // 画像をロード
    const playerImage = new Image();
    playerImage.src = 'images/player.png';

    const bulletImage = new Image();
    bulletImage.src = 'images/bullet.png';

    const player = new Player(400, 500);
    let bullets = [];

    // 画像がロードされたらゲームループを開始
    playerImage.onload = () => {
        bulletImage.onload = () => {
            gameLoop();
        };
    };

    function gameLoop() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // プレイヤーの位置を取得してプレイヤー画像を描画
        const position = player.get_position();
        const px = position[0];
        const py = position[1];
        ctx.drawImage(playerImage, px - playerImage.width / 2, py - playerImage.height / 2);

        // 弾の位置を取得して銃弾画像を描画
        for (let bullet of bullets) {
            bullet.move_up();
            const position = bullet.get_position();
            const bx = position[0];
            const by = position[1];
            ctx.drawImage(bulletImage, bx - bulletImage.width / 2, by - bulletImage.height / 2);
        }

        requestAnimationFrame(gameLoop);
    }

    document.addEventListener('keydown', (event) => {
        if (event.key === 'ArrowLeft') {
            player.move_left();
        } else if (event.key === 'ArrowRight') {
            player.move_right();
        } else if (event.key === ' ') {
            const position = player.get_position();
            const px = position[0];
            const py = position[1];
            bullets.push(new Bullet(px, py - 25));
        }
    });
}

runGame();
