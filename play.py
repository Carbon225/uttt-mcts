from uttt_mcts import UTTTEnvImpl, PMCTS, MCTS
from rich import print


if __name__ == '__main__':
    g = UTTTEnvImpl()
    mcts = MCTS(env=g, time_budget_s=1)
    pmcts = PMCTS(time_budget_s=1)
    while True:
        action = mcts.run()
        mcts.move_root(action)
        obs, reward, done = g.step(action)
        print('\n[green]X turn')
        print('Tree size:', mcts.tree_size())
        print(f'[green]X root value: {mcts.root_value():.2f}')
        g.render()
        if done:
            break

        action = pmcts.run(g)
        # x1, y1, x2, y2 = [int(x) - 1 for x in input('Enter action: ').strip()]
        # action = x1 * 27 + y1 * 9 + x2 * 3 + y2
        mcts.move_root(action)
        obs, reward, done = g.step(action)
        print('\n[red]O turn')
        print('Tree size:', mcts.tree_size())
        print(f'[red]O root value: {mcts.root_value():.2f}')
        g.render()
        if done:
            break

    if reward > 0:
        print("[green]X wins")
    elif reward < 0:
        print("[red]O wins")
    else:
        print("[yellow]draw")
