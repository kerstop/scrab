
export interface PublicRoom {
    tiles: PublicTile[]
}

export interface PublicTile {
    wall: boolean
    cordinate: string
    x: number
    y: number
}