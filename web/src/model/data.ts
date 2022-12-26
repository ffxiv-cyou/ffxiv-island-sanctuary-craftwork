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

export class DemandUtils {
    /**
     * 从真实需求值获取需求区间值
     * @param val 
     * @returns 
     */
    public static GetDemand(val: number): number {
        if (val >= 18) return 0;
        if (val >= 10) return 1;
        if (val >= 2) return 2;
        if (val >= -6) return 3;
        return 4;
    }

    /**
     * 从需求区间值给出一个真实需求值
     * @param val 
     * @returns 
     */
    public static FromDemand(val: number): number {
        switch (val) {
            case 0: return 25;
            case 1: return 13;
            case 2: return 9;
            case 3: return 0;
            case 4: return -7;
            default: return 9;
        }
    }

    /**
     * 从真实需求变动值获取变动区间值
     * @param val 
     * @returns 
     */
    public static GetDemandChange(val: number): number {
        if (val >= 5) return 2;
        if (val >= 2) return 1;
        if (val >= -1) return 0;
        if (val >= -5) return -1;
        return -2;
    }
}