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
    public static TrimName(name: string) {
        if (name.startsWith("海岛")) return name.slice(2);
        if (name.startsWith("开拓工房")) return name.slice(4);
        return name;
    }
}

export class Utils {
    public static Hex2U8Array(text: string): Uint8Array {
        let array = new Uint8Array(text.length / 2);
        for (let i = 0; i < text.length; i += 2) {
            array[i / 2] = parseInt(text.substring(i, i + 2), 16);
        }
        return array;
    }
}