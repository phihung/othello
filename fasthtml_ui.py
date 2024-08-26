from fasthtml.common import (
    fast_app,
    Div,
    serve,
    Script,
    Span,
)
from othello import Board, AlphaBeta


app, rt = fast_app(
    hdrs=[
        Script(src="https://cdn.tailwindcss.com"),
    ],
    pico=False,
    ws_hdr=True,
    live=True,
)
board = Board.default()
bot = AlphaBeta(7)


@rt("/")
def get():
    state = board.state
    cells = [make_cell(state.cells[i], i) for i in range(64)]

    return Div(
        Div(
            Div(
                "Black:",
                make_score(state.black_score, "black-score"),
                cls="bg-black text-white w-32 h-12 text-center content-center shadow-md",
            ),
            Div(make_status(state), cls="content-center"),
            Div(
                "White:",
                make_score(state.white_score, "white-score"),
                cls="bg-white text-black w-32 h-12 text-center content-center shadow-md",
            ),
            cls="mx-auto flex w-[32rem] justify-between",
        ),
        Div(
            *cells,
            cls="mx-auto mt-5 grid w-[32rem] grid-cols-8 gap-0 bg-green-300",
            hx_ext="ws",
            ws_connect="/wscon",
        ),
        cls="m-auto max-w-2xl bg-gray-200 p-12 mt-12",
    )


def make_cell(v, pos):
    stone = None
    if v == "?":
        stone = Div(
            hx_trigger="click",
            hx_vals=f'{{"pos": {pos}}}',
            ws_send=True,
            hx_swap_oob="true",
            id=f"cell-{pos}",
            # cls="h-16 w-16 cursor-pointer bg-purple-200 hover:bg-purple-400",
            cls="mx-2 my-2 h-12 w-12 rounded-full cursor-pointer bg-purple-200 hover:bg-purple-300",
        )
    elif v == "B":
        stone = Div(
            cls="mx-2 my-2 h-12 w-12 rounded-full bg-black shadow-sm shadow-white"
        )
    elif v == "W":
        stone = Div(
            cls="mx-2 my-2 h-12 w-12 rounded-full bg-white shadow-sm shadow-black"
        )
    return Div(
        stone,
        id=f"cell-{pos}",
        cls="h-16 w-16 border border-sky-100",
        hx_swap_oob="true",
    )


def make_score(v, id):
    return Span(v, id=id, hx_swap_oob="true")


def make_status(state):
    status = "Black turn"
    if state.ended:
        if state.white_score > state.black_score:
            status = "White won!"
        elif state.white_score < state.black_score:
            status = "Black won!"
        else:
            status = "Game draw!"
    elif state.player == "W":
        status = "White turn"
    return Span(status, id="status", hx_swap_oob="true")


@app.ws("/wscon")
async def ws(pos: int, send):
    # Human
    state = await move(pos, send)
    if state.ended:
        return

    # Bot
    while True:
        pos = bot.find_move(board) if state.can_move else -1
        state = await move(pos, send)
        if not state.can_move and not state.ended:
            # Human has no move
            await move(-1, send)
        else:
            break


async def move(pos: int, send):
    prev_state = board.state
    state = board.make_move(pos) if pos >= 0 else board.pass_move()

    await send(make_cell(state.cells[pos], pos))
    # await asyncio.sleep(1)

    for i, (c1, c2) in enumerate(zip(prev_state.cells, (state.cells))):
        if i != pos and c1 != c2:
            await send(make_cell(c2, i))

    await send(make_score(state.white_score, "white-score"))
    await send(make_score(state.black_score, "black-score"))
    await send(make_status(state))
    return state


serve()
