import Grass from "./grass";
import Mountain from "./mountain";
import Ocean from "./ocean";

export const generateTilesInMap = (map) => {
  const { ...newMap } = map;
  newMap.tiles = [];
  map.tiles.forEach((row, i) => {
    newMap.tiles.push([]);
    row.forEach((tile, j) => {
      switch (tile.id) {
        case 0: // ocean
          newMap.tiles[i].push(new Ocean(0, i, j, tile.state));
          break;
        case 1: //mountain
          newMap.tiles[i].push(new Mountain(0, i, j, tile.state));
          break;
        case 2: //grass
        default:
          newMap.tiles[i].push(new Grass(0, i, j, tile.state));
          break;
      }
    });
  });
  return newMap;
};
