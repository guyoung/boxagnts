import { u as useSettingsStore } from "./settings-jnLiHyJO.js";
import { R as defineComponent, a9 as useAppStore, O as createElementBlock, L as createBaseVNode, Q as createVNode, ag as withCtx, q as VIcon, e as VCard, d as VBtn, a1 as openBlock, P as createTextVNode, i as VCardText, H as VTextarea, a8 as unref } from "./index-CD7sFTTo.js";
const _hoisted_1 = { class: "d-flex align-center mb-2" };
const _hoisted_2 = { class: "d-flex justify-end mt-4" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsPromptPage",
  setup(__props) {
    const settingsStore = useSettingsStore();
    const appStore = useAppStore();
    function handleSave() {
      appStore.showMessage("System prompt saved locally!", "success");
    }
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            color: "primary",
            class: "mr-2"
          }, {
            default: withCtx(() => [..._cache[1] || (_cache[1] = [
              createTextVNode("mdi-text", -1)
            ])]),
            _: 1
          }),
          _cache[2] || (_cache[2] = createBaseVNode("h2", { class: "text-h5 font-weight-bold" }, "System Prompt", -1))
        ]),
        _cache[5] || (_cache[5] = createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mb-4" }, " Configure the default system prompt that will be used for all AI interactions. ", -1)),
        createVNode(VCard, null, {
          default: withCtx(() => [
            createVNode(VCardText, null, {
              default: withCtx(() => [
                createVNode(VTextarea, {
                  modelValue: unref(settingsStore).settings.system_prompt,
                  "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => unref(settingsStore).settings.system_prompt = $event),
                  variant: "outlined",
                  rows: "10",
                  placeholder: "You are a helpful AI assistant..."
                }, null, 8, ["modelValue"])
              ]),
              _: 1
            })
          ]),
          _: 1
        }),
        createBaseVNode("div", _hoisted_2, [
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            onClick: handleSave
          }, {
            default: withCtx(() => [
              createVNode(VIcon, { start: "" }, {
                default: withCtx(() => [..._cache[3] || (_cache[3] = [
                  createTextVNode("mdi-content-save", -1)
                ])]),
                _: 1
              }),
              _cache[4] || (_cache[4] = createTextVNode(" Save ", -1))
            ]),
            _: 1
          })
        ])
      ]);
    };
  }
});
export {
  _sfc_main as default
};
