import CraftObjects from "@/data/MJICraftworksObject.json";
import ItemPouch from "@/data/MJIItemPouch.json";

export class CraftworkObject {
    Id!: number;
    // 产物名称
    Name!: string;
    // 图标
    Icon!: number;
    // 基础价格
    Price!: number;
    // 成本
    Cost!: number;
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
    // 图标
    Icon!: number;
}

export enum ItemCategory {
    // 未知
    Unknown,
    // 采集物品
    Gathering,
    // 远征
    Explore,
    // 种子
    Seed,
    // 农场产品
    LandProduct,
    // 牧场产品
    FramProduct,
    // 饲料
    Feed,
    // 捕兽
    Catch,
}

export const PatternNames: string[] = 
[
    "未知",
    "2强",
    "2弱",
    "3强",
    "3弱",
    "4强",
    "4弱",
    "5强",
    "5弱",
    "6强",
    "6弱",
    "7强",
    "7弱",
];

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

export class MJIItemData {
    public static GetItem(id: number): ItemName {
        return ItemPouch[id];
    }
    public static TrimName(name: string) {
        if (name.startsWith("海岛")) return name.slice(2);
        if (name.startsWith("牧场动物的")) return name.slice(5);
        if (name.startsWith("无人岛")) return name.slice(3);
        return name;
    }
    public static GetCategory(id: number): ItemCategory {
        if (id < 0) return ItemCategory.Unknown;
        if (id <= 26) return ItemCategory.Gathering;
        if (id <= 31) return ItemCategory.Explore;
        if (id <= 41) return ItemCategory.Seed;
        if (id <= 51) return ItemCategory.LandProduct;
        if (id <= 60) return ItemCategory.FramProduct;
        if (id <= 63) return ItemCategory.Feed;
        if (id <= 66) return ItemCategory.Catch;
        return ItemCategory.Unknown;
    }
}

export class Utils {
    public static Hex2U8Array(text: string): Uint8Array {
        const array = new Uint8Array(text.length / 2);
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
            case 0: return 24;
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