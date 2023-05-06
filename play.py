from uttt_rl import UTTTEnvImpl, PMCTS, MCTS
from rich import print


if __name__ == '__main__':
    g = UTTTEnvImpl()
    mcts = MCTS(env=g, time_budget_s=0.1)
    pmcts = PMCTS(time_budget_s=0.5)
    while True:
        print('\n[green]X[/green] turn')
        action = mcts.run()
        mcts.move_root(action)
        obs, reward, done = g.step(action)
        print('Tree size:', mcts.tree_size())
        g.render()
        if done:
            break

        print('\n[red]O[/red] turn')
        action = pmcts.run(g)
        mcts.move_root(action)
        obs, reward, done = g.step(action)
        print('Tree size:', mcts.tree_size())
        g.render()
        if done:
            break

    if reward > 0:
        print("[green]X wins")
    elif reward < 0:
        print("[red]O wins")
    else:
        print("[yellow]draw")
