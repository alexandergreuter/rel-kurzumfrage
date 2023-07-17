import { get } from '../http';
import { Location } from '../dto/location';

export async function getLocation(id: string): Promise<Location> {
    return await get("/locations/" + id)
}

export async function getLocations(): Promise<Location[]>{
    return await get("/locations/")
}