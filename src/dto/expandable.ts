
export interface ExpandItem{
    name:string,
    description?:string,
    icon?:string,
    children:ExpandItemChild[],
    menuItems?: ExpandItemMenu<ExpandItem>[],
}

export interface ExpandItemChild{
    name:string,
    description?:string,
    icon?:string,
    onClick:()=>void,
    menuItems?:ExpandItemMenu<{item:ExpandItem,child:ExpandItemChild}>[]
}

export interface ExpandItemMenu<I>{
    text:string,
    icon?:string,
    onClick:(item:I)=>void
}