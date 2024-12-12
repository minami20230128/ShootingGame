import run, { Player, Bullet, Enemy } from './pkg/shooting_game.js';

async function runGame() {
    await run(); // wasmの初期化
    const canvas = document.getElementById('gameCanvas');
    const ctx = canvas.getContext('2d');

    // 画像をロード
    const playerImage = new Image();
    playerImage.src = 'images/player.png';
    const bulletImage = new Image();
    bulletImage.src = 'images/bullet.png';
    const enemyImage = new Image();
    enemyImage.src = 'images/enemy.png';

    playerImage.onload = () => {
        bulletImage.onload = () => {
            enemyImage.onload = () => {
                // プレイヤーのインスタンスを作成（画像サイズも渡す）
                const player = new Player(400, 500, playerImage.width, playerImage.height);
                let bullets = [];
                let enemies = [];
                let score = 0;

                // 敵を定期的に生成
                setInterval(() => {
                    const x = Math.random() * canvas.width;
                    enemies.push(new Enemy(x, 0));
                }, 2000);

                function gameLoop() {
                    ctx.clearRect(0, 0, canvas.width, canvas.height);

                    // プレイヤーの位置を取得してプレイヤー画像を描画
                    const position = player.get_position();
                    const px = position[0];
                    const py = position[1];
                    ctx.drawImage(playerImage, px - playerImage.width / 2, py - playerImage.height / 2);

                    // 弾の位置を取得して銃弾画像を描画
                    bullets = bullets.filter((bullet) => {
                        bullet.move_up();
                        const position = bullet.get_position();
                        const bx = position[0];
                        const by = position[1];
                        ctx.drawImage(bulletImage, bx - bulletImage.width / 2, by - bulletImage.height / 2);

                        return by > 0; // 画面外の弾を削除
                    });

                    // 敵の位置を取得して敵画像を描画
                    enemies = enemies.filter((enemy) => {
                        enemy.move_down();
                        const position = enemy.get_position();
                        const ex = position[0];
                        const ey = position[1];
                        ctx.drawImage(enemyImage, ex - enemyImage.width / 2, ey - enemyImage.height / 2);

                        return ey < canvas.height; // 画面外の敵を削除
                    });

                    // 衝突判定
                    bullets.forEach((bullet, index) => {
                        const bulletPosition = bullet.get_position();
                        const bx = bulletPosition[0];
                        const by = bulletPosition[1];

                        enemies.forEach((enemy, enemyIndex) => {
                            const enemyPosition = enemy.get_position();
                            const ex = enemyPosition[0];
                            const ey = enemyPosition[1];

                            // 弾と敵の衝突判定 (簡略化: 画像の中心位置とサイズを基準に当たり判定)
                            const distance = Math.sqrt((bx - ex) ** 2 + (by - ey) ** 2);
                            const threshold = 250; // 衝突判定の範囲 (適宜調整)

                            if (distance < threshold) {
                                // 衝突した場合、敵を消す
                                enemies.splice(enemyIndex, 1);
                                bullets.splice(index, 1);

                                // 得点を加算
                                score += 10;
                            }
                        });
                    });

                    // 敵とプレイヤーの衝突判定
                    enemies.forEach((enemy, enemyIndex) => {
                        const enemyPosition = enemy.get_position();
                        const ex = enemyPosition[0];
                        const ey = enemyPosition[1];

                        const playerPosition = player.get_position();
                        const px = playerPosition[0];
                        const py = playerPosition[1];

                        // プレイヤーと敵の衝突判定
                        const distance = Math.sqrt((px - ex) ** 2 + (py - ey) ** 2);
                        const threshold = 250; // 衝突判定の範囲 (適宜調整)

                        if (distance < threshold) {
                            // プレイヤーと敵が衝突した場合、ライフを減らす
                            player.decrease_life();
                            enemies.splice(enemyIndex, 1);

                            // ライフが0になったらゲームオーバー
                            if (player.get_life() === 0) {
                                alert("Game Over!");
                                // ゲームを終了する処理（例: ループを停止）
                            }
                        }
                    });

                    // 得点とライフ表示
                    ctx.font = '20px Arial';
                    ctx.fillStyle = 'white';
                    ctx.fillText(`Score: ${score}`, canvas.width - 120, canvas.height - 20);
                    ctx.fillText(`Life: ${player.get_life()}`, 20, canvas.height - 20);

                    // 次のフレーム
                    requestAnimationFrame(gameLoop);
                }

                document.addEventListener('keydown', (event) => {
                    if (event.key === 'ArrowLeft') {
                        player.move_left();
                    } else if (event.key === 'ArrowRight') {
                        // canvas.widthを渡す
                        console.log(canvas.width);
                        player.move_right(canvas.width); 
                    } else if (event.key === ' ') {
                        const position = player.get_position();
                        const px = position[0];
                        const py = position[1];
                        bullets.push(new Bullet(px, py - 25));
                    }
                });

                gameLoop(); // ゲームループを開始
            };
        };
    };
}

runGame();
