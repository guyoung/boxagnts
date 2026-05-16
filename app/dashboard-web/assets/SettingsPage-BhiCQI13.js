import { O as defineComponent, a5 as useAppStore, Y as onMounted, L as createElementBlock, I as createBaseVNode, N as createVNode, ac as withCtx, p as VIcon, x as VRow, m as VDialog, $ as ref, G as api, Z as openBlock, M as createTextVNode, l as VCol, d as VCard, i as VCardTitle, h as VCardText, D as VTextarea, q as VList, F as Fragment, a0 as renderList, J as createBlock, u as VListItemTitle, a2 as toDisplayString, c as VBtn, s as VListItem, K as createCommentVNode, y as VSelect, C as VTextField, ae as withKeys, e as VCardActions, A as VSpacer, H as computed } from "./index-orSBHcqs.js";
const _hoisted_1 = { class: "d-flex align-center mb-6" };
const _hoisted_2 = { class: "d-flex justify-end mt-4" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsPage",
  setup(__props) {
    const appStore = useAppStore();
    const saving = ref(false);
    const providerDialogOpen = ref(false);
    const editingProvider = ref(null);
    const passwordVisible = ref(false);
    const defaultSettings = {
      system_prompt: "",
      model_providers: [],
      default_model: "",
      default_provider: "",
      allowed_outbound_hosts: []
    };
    const settings = ref({ ...defaultSettings });
    const newOutboundHost = ref("");
    const providerForm = ref({
      name: "",
      api_key: "",
      base_url: "",
      models: []
    });
    const modelsInput = ref("");
    const providerOptions = computed(() => {
      if (!settings.value.model_providers) return [];
      return settings.value.model_providers.map((p) => ({
        title: p.name,
        value: p.id
      }));
    });
    const modelOptions = computed(() => {
      if (!settings.value.default_provider || !settings.value.model_providers) return [];
      const provider = settings.value.model_providers.find((p) => p.id === settings.value.default_provider);
      return (provider == null ? void 0 : provider.models) || [];
    });
    async function loadSettings() {
      try {
        const data = await api.getSettings();
        settings.value = { ...defaultSettings, ...data };
      } catch (e) {
        console.error("Failed to load settings:", e);
      }
    }
    async function saveSettings() {
      saving.value = true;
      try {
        await api.saveSettings(settings.value);
        appStore.showMessage("Settings saved!", "success");
      } catch {
        appStore.showMessage("Failed to save settings", "error");
      } finally {
        saving.value = false;
      }
    }
    function openAddProviderDialog() {
      editingProvider.value = null;
      providerForm.value = {
        name: "",
        api_key: "",
        base_url: "",
        models: []
      };
      modelsInput.value = "";
      providerDialogOpen.value = true;
    }
    function editProvider(provider) {
      editingProvider.value = { ...provider };
      providerForm.value = { ...provider };
      modelsInput.value = provider.models.join(", ");
      providerDialogOpen.value = true;
    }
    function closeProviderDialog() {
      providerDialogOpen.value = false;
      editingProvider.value = null;
      providerForm.value = {};
      modelsInput.value = "";
    }
    function saveProvider() {
      if (!providerForm.value.name) return;
      const newProvider = {
        id: editingProvider.value ? editingProvider.value.id : "provider_" + Date.now(),
        name: providerForm.value.name || "",
        api_key: providerForm.value.api_key,
        base_url: providerForm.value.base_url,
        models: modelsInput.value.split(",").map((m) => m.trim()).filter(Boolean)
      };
      if (!settings.value.model_providers) {
        settings.value.model_providers = [];
      }
      if (editingProvider.value) {
        const idx = settings.value.model_providers.findIndex((p) => p.id === editingProvider.value.id);
        if (idx >= 0) {
          settings.value.model_providers[idx] = newProvider;
        }
      } else {
        settings.value.model_providers.push(newProvider);
      }
      closeProviderDialog();
    }
    function deleteProvider(id) {
      if (!settings.value.model_providers) return;
      settings.value.model_providers = settings.value.model_providers.filter((p) => p.id !== id);
      if (settings.value.default_provider === id) {
        settings.value.default_provider = "";
        settings.value.default_model = "";
      }
    }
    function addOutboundHost() {
      const host = newOutboundHost.value.trim();
      if (!host) return;
      if (!settings.value.allowed_outbound_hosts) {
        settings.value.allowed_outbound_hosts = [];
      }
      if (!settings.value.allowed_outbound_hosts.includes(host)) {
        settings.value.allowed_outbound_hosts.push(host);
      }
      newOutboundHost.value = "";
    }
    function deleteOutboundHost(idx) {
      if (!settings.value.allowed_outbound_hosts) return;
      settings.value.allowed_outbound_hosts.splice(idx, 1);
    }
    onMounted(() => {
      loadSettings();
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            size: "32",
            color: "primary",
            class: "mr-3"
          }, {
            default: withCtx(() => [..._cache[10] || (_cache[10] = [
              createTextVNode("mdi-cog", -1)
            ])]),
            _: 1
          }),
          _cache[11] || (_cache[11] = createBaseVNode("h1", { class: "text-h4 font-weight-bold" }, "Settings", -1))
        ]),
        createVNode(VRow, null, {
          default: withCtx(() => [
            createVNode(VCol, { cols: "12" }, {
              default: withCtx(() => [
                createVNode(VCard, { class: "mb-4" }, {
                  default: withCtx(() => [
                    createVNode(VCardTitle, { class: "d-flex align-center" }, {
                      default: withCtx(() => [
                        createVNode(VIcon, { start: "" }, {
                          default: withCtx(() => [..._cache[12] || (_cache[12] = [
                            createTextVNode("mdi-text", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[13] || (_cache[13] = createTextVNode(" System Prompt ", -1))
                      ]),
                      _: 1
                    }),
                    createVNode(VCardText, null, {
                      default: withCtx(() => [
                        createVNode(VTextarea, {
                          modelValue: settings.value.system_prompt,
                          "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => settings.value.system_prompt = $event),
                          variant: "outlined",
                          rows: "6",
                          placeholder: "You are a helpful AI assistant..."
                        }, null, 8, ["modelValue"])
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCard, { class: "mb-4" }, {
                  default: withCtx(() => [
                    createVNode(VCardTitle, { class: "d-flex align-center" }, {
                      default: withCtx(() => [
                        createVNode(VIcon, { start: "" }, {
                          default: withCtx(() => [..._cache[14] || (_cache[14] = [
                            createTextVNode("mdi-robot", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[15] || (_cache[15] = createTextVNode(" Model Providers ", -1))
                      ]),
                      _: 1
                    }),
                    createVNode(VCardText, null, {
                      default: withCtx(() => [
                        createVNode(VList, {
                          density: "compact",
                          class: "mb-4"
                        }, {
                          default: withCtx(() => [
                            (openBlock(true), createElementBlock(Fragment, null, renderList(settings.value.model_providers, (provider) => {
                              return openBlock(), createBlock(VListItem, {
                                key: provider.id,
                                rounded: "lg"
                              }, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, null, {
                                    default: withCtx(() => [..._cache[16] || (_cache[16] = [
                                      createTextVNode("mdi-server", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createVNode(VBtn, {
                                    icon: "mdi-pencil",
                                    variant: "text",
                                    onClick: ($event) => editProvider(provider)
                                  }, null, 8, ["onClick"]),
                                  createVNode(VBtn, {
                                    icon: "mdi-delete",
                                    variant: "text",
                                    color: "error",
                                    onClick: ($event) => deleteProvider(provider.id)
                                  }, null, 8, ["onClick"])
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, null, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(provider.name), 1)
                                    ]),
                                    _: 2
                                  }, 1024)
                                ]),
                                _: 2
                              }, 1024);
                            }), 128)),
                            !settings.value.model_providers || settings.value.model_providers.length === 0 ? (openBlock(), createBlock(VListItem, { key: 0 }, {
                              default: withCtx(() => [
                                createVNode(VListItemTitle, { class: "text-medium-emphasis" }, {
                                  default: withCtx(() => [..._cache[17] || (_cache[17] = [
                                    createTextVNode("No providers configured", -1)
                                  ])]),
                                  _: 1
                                })
                              ]),
                              _: 1
                            })) : createCommentVNode("", true)
                          ]),
                          _: 1
                        }),
                        createVNode(VBtn, {
                          color: "primary",
                          "prepend-icon": "mdi-plus",
                          onClick: openAddProviderDialog
                        }, {
                          default: withCtx(() => [..._cache[18] || (_cache[18] = [
                            createTextVNode(" Add Provider ", -1)
                          ])]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCard, { class: "mb-4" }, {
                  default: withCtx(() => [
                    createVNode(VCardTitle, { class: "d-flex align-center" }, {
                      default: withCtx(() => [
                        createVNode(VIcon, { start: "" }, {
                          default: withCtx(() => [..._cache[19] || (_cache[19] = [
                            createTextVNode("mdi-star", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[20] || (_cache[20] = createTextVNode(" Default Settings ", -1))
                      ]),
                      _: 1
                    }),
                    createVNode(VCardText, null, {
                      default: withCtx(() => [
                        createVNode(VSelect, {
                          modelValue: settings.value.default_provider,
                          "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => settings.value.default_provider = $event),
                          label: "Default Provider",
                          items: providerOptions.value,
                          variant: "outlined",
                          class: "mb-4",
                          disabled: !providerOptions.value.length
                        }, null, 8, ["modelValue", "items", "disabled"]),
                        createVNode(VSelect, {
                          modelValue: settings.value.default_model,
                          "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => settings.value.default_model = $event),
                          label: "Default Model",
                          items: modelOptions.value,
                          variant: "outlined",
                          class: "mb-4",
                          disabled: !modelOptions.value.length
                        }, null, 8, ["modelValue", "items", "disabled"])
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCard, null, {
                  default: withCtx(() => [
                    createVNode(VCardTitle, { class: "d-flex align-center" }, {
                      default: withCtx(() => [
                        createVNode(VIcon, { start: "" }, {
                          default: withCtx(() => [..._cache[21] || (_cache[21] = [
                            createTextVNode("mdi-security", -1)
                          ])]),
                          _: 1
                        }),
                        _cache[22] || (_cache[22] = createTextVNode(" Allowed Outbound Hosts ", -1))
                      ]),
                      _: 1
                    }),
                    createVNode(VCardText, null, {
                      default: withCtx(() => [
                        createVNode(VList, {
                          density: "compact",
                          class: "mb-4"
                        }, {
                          default: withCtx(() => [
                            (openBlock(true), createElementBlock(Fragment, null, renderList(settings.value.allowed_outbound_hosts, (host, idx) => {
                              return openBlock(), createBlock(VListItem, {
                                key: idx,
                                rounded: "lg"
                              }, {
                                prepend: withCtx(() => [
                                  createVNode(VIcon, null, {
                                    default: withCtx(() => [..._cache[23] || (_cache[23] = [
                                      createTextVNode("mdi-server", -1)
                                    ])]),
                                    _: 1
                                  })
                                ]),
                                append: withCtx(() => [
                                  createVNode(VBtn, {
                                    icon: "mdi-delete",
                                    variant: "text",
                                    color: "error",
                                    onClick: ($event) => deleteOutboundHost(idx)
                                  }, null, 8, ["onClick"])
                                ]),
                                default: withCtx(() => [
                                  createVNode(VListItemTitle, null, {
                                    default: withCtx(() => [
                                      createTextVNode(toDisplayString(host), 1)
                                    ]),
                                    _: 2
                                  }, 1024)
                                ]),
                                _: 2
                              }, 1024);
                            }), 128)),
                            !settings.value.allowed_outbound_hosts || settings.value.allowed_outbound_hosts.length === 0 ? (openBlock(), createBlock(VListItem, { key: 0 }, {
                              default: withCtx(() => [
                                createVNode(VListItemTitle, { class: "text-medium-emphasis" }, {
                                  default: withCtx(() => [..._cache[24] || (_cache[24] = [
                                    createTextVNode("No outbound hosts configured", -1)
                                  ])]),
                                  _: 1
                                })
                              ]),
                              _: 1
                            })) : createCommentVNode("", true)
                          ]),
                          _: 1
                        }),
                        createVNode(VRow, {
                          dense: "",
                          align: "center"
                        }, {
                          default: withCtx(() => [
                            createVNode(VCol, { cols: "10" }, {
                              default: withCtx(() => [
                                createVNode(VTextField, {
                                  modelValue: newOutboundHost.value,
                                  "onUpdate:modelValue": _cache[3] || (_cache[3] = ($event) => newOutboundHost.value = $event),
                                  label: "Add Host",
                                  variant: "outlined",
                                  placeholder: "api.example.com",
                                  onKeyup: withKeys(addOutboundHost, ["enter"])
                                }, null, 8, ["modelValue"])
                              ]),
                              _: 1
                            }),
                            createVNode(VCol, { cols: "2" }, {
                              default: withCtx(() => [
                                createVNode(VBtn, {
                                  color: "primary",
                                  block: "",
                                  onClick: addOutboundHost,
                                  disabled: !newOutboundHost.value.trim()
                                }, {
                                  default: withCtx(() => [..._cache[25] || (_cache[25] = [
                                    createTextVNode(" Add ", -1)
                                  ])]),
                                  _: 1
                                }, 8, ["disabled"])
                              ]),
                              _: 1
                            })
                          ]),
                          _: 1
                        })
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
                    loading: saving.value,
                    onClick: saveSettings
                  }, {
                    default: withCtx(() => [
                      createVNode(VIcon, { start: "" }, {
                        default: withCtx(() => [..._cache[26] || (_cache[26] = [
                          createTextVNode("mdi-content-save", -1)
                        ])]),
                        _: 1
                      }),
                      _cache[27] || (_cache[27] = createTextVNode(" Save Settings ", -1))
                    ]),
                    _: 1
                  }, 8, ["loading"])
                ])
              ]),
              _: 1
            })
          ]),
          _: 1
        }),
        createVNode(VDialog, {
          modelValue: providerDialogOpen.value,
          "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => providerDialogOpen.value = $event),
          "max-width": "500"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(editingProvider.value ? "Edit Provider" : "Add Provider"), 1)
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: providerForm.value.name,
                              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => providerForm.value.name = $event),
                              label: "Name",
                              variant: "outlined",
                              placeholder: "e.g., OpenAI"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: providerForm.value.api_key,
                              "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => providerForm.value.api_key = $event),
                              label: "API Key",
                              variant: "outlined",
                              type: passwordVisible.value ? "text" : "password",
                              "append-inner-icon": passwordVisible.value ? "mdi-eye-off" : "mdi-eye",
                              "onClick:appendInner": _cache[6] || (_cache[6] = ($event) => passwordVisible.value = !passwordVisible.value)
                            }, null, 8, ["modelValue", "type", "append-inner-icon"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: providerForm.value.base_url,
                              "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => providerForm.value.base_url = $event),
                              label: "Base URL",
                              variant: "outlined",
                              placeholder: "https://api.openai.com/v1"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: modelsInput.value,
                              "onUpdate:modelValue": _cache[8] || (_cache[8] = ($event) => modelsInput.value = $event),
                              label: "Models",
                              variant: "outlined",
                              placeholder: "gpt-4o, gpt-4, gpt-3.5-turbo",
                              hint: "Comma-separated list of model names",
                              "persistent-hint": ""
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: closeProviderDialog
                    }, {
                      default: withCtx(() => [..._cache[28] || (_cache[28] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      onClick: saveProvider
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(editingProvider.value ? "Update" : "Add"), 1)
                      ]),
                      _: 1
                    })
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"])
      ]);
    };
  }
});
export {
  _sfc_main as default
};
