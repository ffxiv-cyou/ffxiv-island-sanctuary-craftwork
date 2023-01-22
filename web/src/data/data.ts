import CraftObjectsCN from "./cn/MJICraftworksObject.json";
import ItemPouchCN from "./cn/MJIItemPouch.json";
import PopularityCN from "@/data/cn/MJICraftworksPopularity.json";
import CraftObjectsGlobal from "./global/MJICraftworksObject.json";
import ItemPouchGlobal from "./global/MJIItemPouch.json";
import PopularityGlobal from "@/data/global/MJICraftworksPopularity.json";

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
    // 分类
    Category!: number;
}

export enum ItemCategory {
    // 未知
    Unknown,
    // 采集物品
    Material,
    // 远征
    RareMaterial,
    // 种子
    Seed,
    // 农场产品
    Produce,
    // 牧场产品
    Leavings,
    // 饲料
    Feed,
    // 捕兽
    Restraint,
}

export enum Region {
    Global,
    CN
}

export class CraftworkData {

    region!: Region;

    constructor(region: Region) {
        this.region = region;
    }

    public SetRegion(region: Region) {
        this.region = region;
    }

    /**
     * 配方表
     */
    get Recipes(): CraftworkObject[] {
        switch (this.region) {
            case Region.Global:
                return CraftObjectsGlobal;
            case Region.CN:
                return CraftObjectsCN;
            default:
                return [];
        }
    }

    /**
     * 原料表
     */
    get Items(): ItemName[] {
        switch (this.region) {
            case Region.Global:
                return ItemPouchGlobal;
            case Region.CN:
                return ItemPouchCN;
            default:
                return [];
        }
    }

    get Popularity(): number[][] {
        switch (this.region) {
            case Region.Global:
                return PopularityGlobal;
            case Region.CN:
                return PopularityCN;
            default:
                return [];
        }
    }

    /**
     * 获取指定ID的配方
     * @param global 区域
     * @param id 配方ID
     * @returns 配方
     */
    public GetRecipe(id: number): CraftworkObject {
        return this.Recipes[id];
    }

    /**
     * 缩短名字长度
     * @param name 原名字
     * @returns 新名字
     */
    public static TrimName(name: string) {
        if (name.startsWith("海岛")) return name.slice(2);
        if (name.startsWith("开拓工房")) return name.slice(4);
        if (name.startsWith("牧场动物的")) return name.slice(5);
        if (name.startsWith("无人岛")) return name.slice(3);
        if (name.startsWith("Island")) return name.slice(7);
        if (name.startsWith("Sanctuary")) return name.slice(10);
        if (name.startsWith("Isleworks")) return name.slice(10);
        return name;
    }

    /**
     * 获取原料
     * @param id 
     * @returns 
     */
    public GetItem(id: number): ItemName {
        return this.Items[id];
    }
}
