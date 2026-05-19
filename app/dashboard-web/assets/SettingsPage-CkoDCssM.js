import { R as defineComponent, af as watch, O as createElementBlock, L as createBaseVNode, Q as createVNode, ag as withCtx, q as VIcon, E as VTabs, a2 as ref, a1 as openBlock, P as createTextVNode, D as VTab, ac as useRouter, ab as useRoute, a4 as resolveComponent } from "./index-CD7sFTTo.js";
const _hoisted_1 = { class: "d-flex align-center mb-6" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsPage",
  setup(__props) {
    const router = useRouter();
    const route = useRoute();
    const activeTab = ref("model");
    function getTabFromPath(path) {
      if (path.includes("/settings/security")) return "security";
      if (path.includes("/settings/prompt")) return "prompt";
      return "model";
    }
    watch(() => route.path, (val) => {
      activeTab.value = getTabFromPath(val);
    }, { immediate: true });
    function navigateTo(tab) {
      router.push(`/settings/${tab}`);
    }
    return (_ctx, _cache) => {
      const _component_router_view = resolveComponent("router-view");
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            size: "32",
            color: "primary",
            class: "mr-3"
          }, {
            default: withCtx(() => [..._cache[4] || (_cache[4] = [
              createTextVNode("mdi-cog", -1)
            ])]),
            _: 1
          }),
          _cache[5] || (_cache[5] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Settings", -1))
        ]),
        createVNode(VTabs, {
          modelValue: activeTab.value,
          "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => activeTab.value = $event),
          color: "primary",
          class: "mb-6"
        }, {
          default: withCtx(() => [
            createVNode(VTab, {
              value: "model",
              "prepend-icon": "mdi-robot",
              onClick: _cache[0] || (_cache[0] = ($event) => navigateTo("model"))
            }, {
              default: withCtx(() => [..._cache[6] || (_cache[6] = [
                createTextVNode(" Model Settings ", -1)
              ])]),
              _: 1
            }),
            createVNode(VTab, {
              value: "prompt",
              "prepend-icon": "mdi-text",
              onClick: _cache[1] || (_cache[1] = ($event) => navigateTo("prompt"))
            }, {
              default: withCtx(() => [..._cache[7] || (_cache[7] = [
                createTextVNode(" System Prompt ", -1)
              ])]),
              _: 1
            }),
            createVNode(VTab, {
              value: "security",
              "prepend-icon": "mdi-security",
              onClick: _cache[2] || (_cache[2] = ($event) => navigateTo("security"))
            }, {
              default: withCtx(() => [..._cache[8] || (_cache[8] = [
                createTextVNode(" Security ", -1)
              ])]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(_component_router_view)
      ]);
    };
  }
});
export {
  _sfc_main as default
};
