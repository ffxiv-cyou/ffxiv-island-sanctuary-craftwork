import PopularSheet from "../data/MJICraftworksPopularity.json";
import CraftObjects from "@/data/MJICraftworksObject.json";

export class CraftworkObject {
    Id!: number;
    // 产物名称
    Name!: string;
    // 基础价格
    Price!: number;
    // 等级
    Level!: number;
    // 所需时间
    Time!: number;
    // 类别1
    Theme0!: number;
    // 类别2
    Theme1!: number;
    // 配方
    Ingredients!: CraftworkIngredient[];
}

export class CraftworkIngredient {
    // 原料ID
    Id!: number;
    // 原料数量
    Count!: number;
}

export class ItemName {
    Id!: number;
    Name!: string;
}

export class CraftworkData {
    public static GetRecipe(id: number): CraftworkObject {
        return CraftObjects[id];
    }
}