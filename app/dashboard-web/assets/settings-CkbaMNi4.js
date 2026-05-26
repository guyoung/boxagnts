import { Q as defineStore, a1 as ref } from "./main-D22gLLWp.js";
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
