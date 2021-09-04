export const setMap = (map) => {
  return {
    type: "SET_MAP",
    payload: {
      map: map,
    },
  };
};
