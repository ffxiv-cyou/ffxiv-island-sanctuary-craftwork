/// <reference lib="webworker" />

import init, {
  init_api_v2,
  type APIv2, CraftworkInfo,
  pattern_predict,
  pattern_demand,
  pattern_predict_adv,
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

  let repo: APIv2;
  self.onmessage = async (evt) => {
    const data = evt.data;
    switch (data.type) {
      case "init_repo":
        repo = init_api_v2(data.recipe, data.pops, data.pop_row);
        self.postMessage({
          type: "init_repo"
        });
        return;
      case "pattern_predict": {
        const result = pattern_predict(data.array, data.demands);
        self.postMessage(result, [result.buffer]);
        return;
      }
      case "pattern_predict_adv": {
        const result = pattern_predict_adv(data.array, data.demands);
        self.postMessage(result, [result.buffer]);
        return;
      }
      case "pattern_demand": {
        const result = pattern_demand(data.array, data.day);
        self.postMessage(result, [result.buffer])
        return;
      }
      default:
        if (!repo) {
          return self.reportError("repo is not inited. " + data.type);
        }
        break;
    }
    switch (data.type) {
      case "set_pattern":
        repo.set_pattern(data.pattern);
        self.postMessage({
          type: "set_pattern"
        });
        break;
      case "solve_week": {
        const state = get_craft_info(data.state);
        const result = repo.solve_week_single(
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
        const result = repo.solve_day_with_batch(
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
        const result = repo.solve_day_dual(
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
      case "solve_day_favor": {
        const state = get_craft_info(data.state);
        const result = repo.solve_day_with_favor(
          state,
          data.level,
          data.ban_list,
          data.set,
          data.demands,
          data.favors,
          data.time,
          data.with_cost);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "solve_week_part": {
        const state = get_craft_info(data.state);
        const result = repo.solve_week_part(
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
      case "solve_cache_clear": {
        repo.solver_clear_cache();
        self.postMessage({
          type: "solve_cache_clear"
        });
        break;
      }
      case "simulate_multi": {
        const state = get_craft_info(data.state);
        const result = repo.simulate(state, data.seq, data.demands);
        self.postMessage(result, [result.buffer]);
        break;
      }
      case "simulate":
        return self.reportError("simulate is deprecated. use simulate_multi instead");
      case "solve_day":
        return self.reportError("solve_day is deprecated. use solve_multi_day instead");
      default:
        console.log("unexpected type", data.type);
        self.reportError("unexpected type " + data.type);
        break;
    }
  }
}

init_worker();
