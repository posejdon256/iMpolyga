import { getNewMap_to_server, initCommuniation } from ".";
import { defaultErrorHandler } from "./error";
import { getMapSuccess } from "./map-callbacks";

export const initialize = async () => {
  initCommuniation();
  getNewMap_to_server(getMapSuccess, defaultErrorHandler);
};
