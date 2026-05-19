import { S as defineStore, a2 as ref } from "./index-CD7sFTTo.js";
const useSettingsStore = defineStore("settings", () => {
  const settings = ref({
    system_prompt: "",
    default_model: "",
    default_provider: "",
    allowed_outbound_hosts: []
  });
  return {
    settings
  };
});
export {
  useSettingsStore as u
};
