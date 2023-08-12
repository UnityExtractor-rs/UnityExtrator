
export interface ExpandItem{
    name:string,
    icon?:string,
    childen:ExpandItemChild[],
    onClick:()=>void
}

export interface ExpandItemChild{
    name:string,
    icon?:string,
    onClick:()=>void
}