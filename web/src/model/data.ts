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
        if (val === -128) return -1; // 未知
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