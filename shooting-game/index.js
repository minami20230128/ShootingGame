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
    const heartImage = new Image();
    heartImage.src = 'images/heart.png'; // ハート画像のパスを指定
    const backgroundImage = new Image();  // 背景画像用のImageオブジェクト
    backgroundImage.src = 'images/background.png';  // 背景画像のパス

    playerImage.onload = () => {
        bulletImage.onload = () => {
            enemyImage.onload = () => {
                heartImage.onload = () => {
                    backgroundImage.onload = () => {  // 背景画像の読み込みが完了した後にゲーム開始
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
                            // 背景画像を描画
                            ctx.clearRect(0, 0, canvas.width, canvas.height);
                            ctx.drawImage(backgroundImage, 0, 0, canvas.width, canvas.height);

                            // プレイヤーの位置を取得してプレイヤー画像を描画
                            const position = player.get_position();
                            const px = position.x; // 修正: positionからxを取得
                            const py = position.y; // 修正: positionからyを取得
                            ctx.drawImage(playerImage, px - playerImage.width / 2, py - playerImage.height / 2);

                            // 弾の位置を取得して銃弾画像を描画
                            bullets = bullets.filter((bullet) => {
                                bullet.move_up();
                                const position = bullet.get_position();
                                const bx = position.x; // 修正: positionからxを取得
                                const by = position.y; // 修正: positionからyを取得
                                ctx.drawImage(bulletImage, bx - bulletImage.width / 2, by - bulletImage.height / 2);

                                return by > 0; // 画面外の弾を削除
                            });

                            // 敵の位置を取得して敵画像を描画
                            enemies = enemies.filter((enemy) => {
                                enemy.move_down();
                                const position = enemy.get_position();
                                const ex = position.x; // 修正: positionからxを取得
                                const ey = position.y; // 修正: positionからyを取得
                                ctx.drawImage(enemyImage, ex - enemyImage.width / 2, ey - enemyImage.height / 2);

                                return ey < canvas.height; // 画面外の敵を削除
                            });

                            // 衝突判定
                            bullets.forEach((bullet, index) => {
                                const bulletPosition = bullet.get_position();
                                const bx = bulletPosition.x; // 修正: positionからxを取得
                                const by = bulletPosition.y; // 修正: positionからyを取得

                                enemies.forEach((enemy, enemyIndex) => {
                                    const enemyPosition = enemy.get_position();
                                    const ex = enemyPosition.x; // 修正: positionからxを取得
                                    const ey = enemyPosition.y; // 修正: positionからyを取得

                                    const distance = Math.sqrt((bx - ex) ** 2 + (by - ey) ** 2);
                                    const threshold = 150;

                                    if (distance < threshold) {
                                        enemies.splice(enemyIndex, 1);
                                        bullets.splice(index, 1);
                                        score += 10;
                                    }
                                });
                            });

                            // 敵とプレイヤーの衝突判定
                            enemies.forEach((enemy, enemyIndex) => {
                                const enemyPosition = enemy.get_position();
                                const ex = enemyPosition.x; // 修正: positionからxを取得
                                const ey = enemyPosition.y; // 修正: positionからyを取得

                                const playerPosition = player.get_position();
                                const px = playerPosition.x; // 修正: positionからxを取得
                                const py = playerPosition.y; // 修正: positionからyを取得

                                const distance = Math.sqrt((px - ex) ** 2 + (py - ey) ** 2);
                                const threshold = 150;

                                if (distance < threshold) {
                                    player.decrease_life();
                                    enemies.splice(enemyIndex, 1);

                                    if (player.get_life() === 0) {
                                        drawLife(ctx, player.get_life(), heartImage, canvas);
                                        alert("Game Over!");
                                    }
                                }
                            });

                            // 得点表示
                            ctx.font = '20px Arial';
                            ctx.fillStyle = 'white';
                            ctx.fillText(`Score: ${score}`, 20, canvas.height - 20);

                            // ライフの数に応じてハート画像を描画
                            const life = player.get_life();
                            drawLife(ctx, life, heartImage, canvas);

                            requestAnimationFrame(gameLoop);
                        }

                        document.addEventListener('keydown', (event) => {
                            if (event.key === 'ArrowLeft') {
                                player.move_left();
                            } else if (event.key === 'ArrowRight') {
                                player.move_right(canvas.width);
                            } else if (event.key === ' ') {
                                const position = player.get_position();
                                const px = position.x; // 修正: positionからxを取得
                                const py = position.y; // 修正: positionからyを取得
                                bullets.push(new Bullet(px, py - 25));
                            }
                        });

                        gameLoop(); // ゲームループを開始
                    };
                };
            };
        };
    };
}

runGame();

function drawLife(ctx, life, heartImage, canvas)
{
    const heartSize = 30; // ハート画像のサイズ

    for (let i = 0; i < life; i++) {
        ctx.drawImage(
            heartImage,
            canvas.width - 30 - (i + 1) * (heartSize + 5), // 右下から左に向けて描画
            canvas.height - heartSize - 10, // 下から少し上に位置を調整
            heartSize,
            heartSize
        );
    }
} 