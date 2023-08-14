
export interface ExpandItem{
    name:string,
    desription?:string,
    icon?:string,
    childen:ExpandItemChild[],
    menuItems?: ExpandItemMenu<ExpandItem>[],
}

export interface ExpandItemChild{
    name:string,
    desription?:string,
    icon?:string,
    onClick:()=>void,
    menuItems?:ExpandItemMenu<{item:ExpandItem,child:ExpandItemChild}>[]
}

export interface ExpandItemMenu<I>{
    text:string,
    icon?:string,
    onClick:(item:I)=>void
}