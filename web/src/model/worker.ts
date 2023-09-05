/// <reference lib="webworker" />

import init, {
  init_repo,
  set_pattern,
  solve_singleday,
  solve_week,
  type GameDataRepo, CraftworkInfo,
  simulate,
  pattern_predict,
  pattern_demand,
  pattern_predict_adv,
  simulate_multi,
  solve_multi_day,
  solve_day_dual,
  solve_week_part,
} from "mji-craftwork";

function get_craft_info(state: any): CraftworkInfo {
  return new CraftworkInfo(
    state.tension,
    state.max_tension,
    state.level,
    state.workers
  );
}

async function init_worker() {
  await init();
  self.postMessage({
    type: "inited",
  });

  let repo: GameDataRepo;
  self.onmessage = async (evt) => {
    const data = evt.data;
    if (data.type !== "init_repo" && !repo) {
      return self.reportError("repo is not inited. " + data.type);
    }

    switch (data.type) {
      case "init_repo":
        repo = init_repo(data.recipe, data.pops, data.pop_row);
        self.postMessage({
          type: "init_repo"
        });
        break;
      case "set_pattern":
        set_pattern(repo, data.pattern);
        self.postMessage({
          type: "set_pattern"
        });
        break;
      case "solve_day": {
        const state = get_craft_info(data.state);
        const result = solve_singleday(repo, state, data.level, data.ban_list, data.demands, data.time, data.with_cost);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "solve_week": {
        const state = get_craft_info(data.state);
        const result = solve_week(
          repo,
          state,
          data.level,
          data.ban_list,
          data.time,
          data.with_cost,
          data.pattern);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "solve_multi_day": {
        const state = get_craft_info(data.state);
        const result = solve_multi_day(
          repo, 
          state, 
          data.level, 
          data.ban_list, 
          data.set, 
          data.demands,
          data.worker, 
          data.time,
          data.with_cost);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "solve_day_dual": {
        const state = get_craft_info(data.state);
        const result = solve_day_dual(
          repo, 
          state, 
          data.level, 
          data.ban_list, 
          data.demands,
          data.worker, 
          data.time,
          data.with_cost);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "solve_week_part": {
        const state = get_craft_info(data.state);
        const result = solve_week_part(
          repo,
          state,
          data.level,
          data.ban_list,
          data.time,
          data.with_cost,
          data.pattern,
          data.part_id);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "simulate": {
        const state = get_craft_info(data.state);
        const result = simulate(
          repo,
          state,
          data.seq,
          data.demands,
        );
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "simulate_multi": {
        const state = get_craft_info(data.state);
        const result = simulate_multi(repo, state, data.seq, data.demands);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "pattern_predict": {
        const result = pattern_predict(data.array, data.demands);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "pattern_predict_adv": {
        const result = pattern_predict_adv(data.array, data.demands);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "pattern_demand": {
        const result = pattern_demand(data.array, data.day);
        self.postMessage(result, [result.buffer])
        break;
      }
      default:
        console.log("unexpected type", data.type);
        self.reportError("unexpected type " + data.type);
        break;
    }
  }
}

init_worker();
