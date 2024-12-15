import init, { Game } from './pkg/shooting_game.js'; // Adjust path based on your build

async function main() {
    // Initialize the WebAssembly module
    await init();

    // Create a new game instance with the canvas ID
    const game = new Game('gameCanvas');

    // Initialize and start the game
    game.init();
}

main().catch(console.error);
