from othello import Game, AlphaBetaBot
import tqdm


def run_match(bot1, bot2):
    game = Game.default()
    while not game.state.ended:
        bot = bot1 if game.state.player == "B" else bot2
        pos = bot.find_move(game)
        if pos >= 0:
            game.make_move(pos)
        else:
            game.pass_move()
    w, b = game.state.white_score, game.state.black_score
    return 1 if b > w else 2 if w > b else 0


def run_matches(bot1, bot2, n):
    cnts = [0, 0, 0]
    for _ in tqdm.tqdm(range(n)):
        result = run_match(bot1, bot2)
        cnts[result] += 1
    return cnts


def _str(bot):
    return f"AlphaBetaBot({bot.depth}, {bot.exhaustive_depth})"


if __name__ == "__main__":
    for i in range(2, 10):
        bot1 = AlphaBetaBot(i, 14)
        bot2 = AlphaBetaBot(i + 1, 14)
        print(f"{_str(bot1)} vs {_str(bot2)})")
        draw, win, lost = run_matches(bot1, bot2, 10)
        print(f"Win: {win} | Draw: {draw} | Lost: {lost}")
        print("----")

        print(f"{_str(bot2)} vs {_str(bot1)})")
        draw, win, lost = run_matches(bot2, bot1, 10)
        print(f"Win: {win} | Draw: {draw} | Lost: {lost}")
        print("----")
