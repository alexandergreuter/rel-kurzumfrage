import { post } from "../http";

export async function submitVote(vote: {
  location_id: string;
  agrees: boolean;
  comment: string | null;
}): Promise<Location> {
  return await post("/votes", vote);
}
