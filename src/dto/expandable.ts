
export interface ExpandItem{
    name:string,
    desription?:string,
    icon?:string,
    childen:ExpandItemChild[],
    menuItems?: ExpandItemMenu[],
}

export interface ExpandItemChild{
    name:string,
    desription?:string,
    icon?:string,
    onClick:()=>void
}

export interface ExpandItemMenu{
    text:string,
    icon?:string,
    onClick:(item:ExpandItem)=>void
}