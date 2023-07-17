import { getLocal, mutateLocal } from "../localstore";

export function hasAlreadyVotedForLocation(locationId: string) {
    return getLocal("votedLocations").some(it => it == locationId);
}

export function addVotedForLocation(locationId: string) {
    mutateLocal("votedLocations", locations => [...locations, locationId])
}