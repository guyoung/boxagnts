import { S as defineComponent, ab as useAppStore, a1 as onMounted, P as createElementBlock, M as createBaseVNode, R as createVNode, ai as withCtx, r as VIcon, f as VCard, e as VBtn, o as VDialog, a4 as ref, K as api, a2 as openBlock, Q as createTextVNode, k as VCardTitle, j as VCardText, N as createBlock, x as VProgressCircular, s as VList, F as Fragment, a5 as renderList, t as VListItem, v as VListItemTitle, a8 as toDisplayString, u as VListItemSubtitle, O as createCommentVNode, m as VChip, al as withModifiers, q as VExpandTransition, aj as withDirectives, ag as vShow, p as VDivider, A as VSelect, aa as unref, z as VRow, H as VTextField, n as VCol, a as VAutocomplete, Y as mergeProps, D as VSwitch, g as VCardActions, C as VSpacer, L as computed } from "./main-BSD2YpbL.js";
import { u as useSettingsStore } from "./settings-RFBLOOFr.js";
const _hoisted_1 = { class: "d-flex align-center mb-2" };
const _hoisted_2 = { class: "text-medium-emphasis" };
const _hoisted_3 = { key: 0 };
const _hoisted_4 = { class: "model-list ml-8 mb-2" };
const _hoisted_5 = {
  key: 0,
  class: "px-3 pb-2"
};
const _hoisted_6 = { class: "px-3 pb-2" };
const _hoisted_7 = { class: "d-flex justify-end mt-4" };
const _hoisted_8 = { class: "d-flex align-center" };
const _sfc_main = /* @__PURE__ */ defineComponent({
  __name: "SettingsModelPage",
  setup(__props) {
    const settings = useSettingsStore();
    const appStore = useAppStore();
    const providers = ref([]);
    const providersLoading = ref(false);
    const providerOptionsData = ref([]);
    const optionsLoading = ref(false);
    const expandedSet = ref(/* @__PURE__ */ new Set());
    const updatingDefault = ref(false);
    function isExpanded(id) {
      return expandedSet.value.has(id);
    }
    function toggleExpand(id) {
      const s = new Set(expandedSet.value);
      if (s.has(id)) {
        s.delete(id);
      } else {
        s.add(id);
      }
      expandedSet.value = s;
    }
    const savingProvider = ref(false);
    const deletingProviderLoading = ref(false);
    const dialogOpen = ref(false);
    const deleteConfirmOpen = ref(false);
    const isEditing = ref(false);
    const passwordVisible = ref(false);
    const editingProviderId = ref(null);
    const deletingProvider = ref(null);
    const form = ref({
      id: "",
      name: "",
      api_base: "",
      api_key: "",
      enabled: true
    });
    const modelDialogOpen = ref(false);
    const deleteModelConfirmOpen = ref(false);
    const isEditingModel = ref(false);
    const savingModel = ref(false);
    const deletingModelLoading = ref(false);
    const editingModelProviderId = ref("");
    const editingModelId = ref("");
    const deletingModelProviderId = ref("");
    const deletingModel = ref(null);
    const modelForm = ref({
      id: "",
      name: ""
    });
    const providerOptionItems = computed(() => providerOptionsData.value);
    const providerOptions = computed(() => {
      return providers.value.map((p) => ({ title: p.name, value: p.id }));
    });
    const modelOptions = computed(() => {
      if (!settings.settings.default_provider) return [];
      const provider = providers.value.find((p) => p.id === settings.settings.default_provider);
      return ((provider == null ? void 0 : provider.models) || []).map((m) => ({ title: m.name, value: m.id }));
    });
    async function fetchProviders() {
      providersLoading.value = true;
      try {
        providers.value = await api.getConfigProviders();
      } catch (e) {
        console.error("Failed to fetch providers:", e);
        providers.value = [];
      } finally {
        providersLoading.value = false;
      }
    }
    async function fetchProviderOptions() {
      optionsLoading.value = true;
      try {
        providerOptionsData.value = await api.getProviderOptions();
      } catch (e) {
        console.error("Failed to fetch provider options:", e);
        providerOptionsData.value = [];
      } finally {
        optionsLoading.value = false;
      }
    }
    async function handleUpdateDefaultModel() {
      if (!settings.settings.default_model) return;
      updatingDefault.value = true;
      try {
        await api.updateDefaultModel(settings.settings.default_model);
        appStore.showMessage("Default model updated!", "success");
      } catch (e) {
        appStore.showMessage("Failed to update default model", "error");
      } finally {
        updatingDefault.value = false;
      }
    }
    function onProviderSelected(id) {
      const option = providerOptionsData.value.find((o) => o.id === id);
      if (option) {
        form.value.name = option.title;
      }
    }
    function openAddDialog() {
      isEditing.value = false;
      editingProviderId.value = null;
      form.value = { id: "", name: "", api_base: "", api_key: "", enabled: true };
      dialogOpen.value = true;
    }
    function openEditDialog(provider) {
      isEditing.value = true;
      editingProviderId.value = provider.id;
      form.value = {
        id: provider.id,
        name: provider.name,
        api_base: provider.api_base,
        api_key: provider.api_key || "",
        enabled: provider.enabled
      };
      dialogOpen.value = true;
    }
    async function handleSaveProvider() {
      if (!form.value.id) return;
      savingProvider.value = true;
      try {
        if (isEditing.value && editingProviderId.value) {
          await api.updateConfigProvider(editingProviderId.value, {
            name: form.value.name,
            api_base: form.value.api_base,
            api_key: form.value.api_key,
            enabled: form.value.enabled
          });
          appStore.showMessage("Provider updated!", "success");
        } else {
          await api.createConfigProvider({
            id: form.value.id,
            name: form.value.name,
            api_base: form.value.api_base,
            api_key: form.value.api_key,
            enabled: form.value.enabled
          });
          appStore.showMessage("Provider added!", "success");
        }
        dialogOpen.value = false;
        await fetchProviders();
      } catch (e) {
        appStore.showMessage("Failed to save provider", "error");
      } finally {
        savingProvider.value = false;
      }
    }
    function confirmDelete(provider) {
      deletingProvider.value = provider;
      deleteConfirmOpen.value = true;
    }
    async function handleDeleteProvider() {
      if (!deletingProvider.value) return;
      deletingProviderLoading.value = true;
      try {
        await api.deleteConfigProvider(deletingProvider.value.id);
        if (settings.settings.default_provider === deletingProvider.value.id) {
          settings.settings.default_provider = "";
          settings.settings.default_model = "";
        }
        appStore.showMessage("Provider deleted!", "success");
        deleteConfirmOpen.value = false;
        await fetchProviders();
      } catch (e) {
        appStore.showMessage("Failed to delete provider", "error");
      } finally {
        deletingProviderLoading.value = false;
      }
    }
    function openAddModelDialog(providerId) {
      isEditingModel.value = false;
      editingModelProviderId.value = providerId;
      editingModelId.value = "";
      modelForm.value = { id: "", name: "" };
      modelDialogOpen.value = true;
    }
    function openEditModelDialog(providerId, model) {
      isEditingModel.value = true;
      editingModelProviderId.value = providerId;
      editingModelId.value = model.id;
      modelForm.value = { id: model.id, name: model.name };
      modelDialogOpen.value = true;
    }
    async function handleSaveModel() {
      if (!modelForm.value.name) return;
      savingModel.value = true;
      try {
        if (isEditingModel.value) {
          await api.updateProviderModel(editingModelProviderId.value, editingModelId.value, {
            name: modelForm.value.name
          });
          appStore.showMessage("Model updated!", "success");
        } else {
          if (!modelForm.value.id) return;
          await api.createProviderModel(editingModelProviderId.value, {
            id: modelForm.value.id,
            name: modelForm.value.name
          });
          appStore.showMessage("Model added!", "success");
        }
        modelDialogOpen.value = false;
        await fetchProviders();
      } catch (e) {
        appStore.showMessage("Failed to save model", "error");
      } finally {
        savingModel.value = false;
      }
    }
    function confirmDeleteModel(providerId, model) {
      deletingModelProviderId.value = providerId;
      deletingModel.value = model;
      deleteModelConfirmOpen.value = true;
    }
    async function handleDeleteModel() {
      if (!deletingModel.value) return;
      deletingModelLoading.value = true;
      try {
        await api.deleteProviderModel(deletingModelProviderId.value, deletingModel.value.id);
        if (settings.settings.default_provider === deletingModelProviderId.value && settings.settings.default_model === deletingModel.value.id) {
          settings.settings.default_model = "";
        }
        appStore.showMessage("Model deleted!", "success");
        deleteModelConfirmOpen.value = false;
        await fetchProviders();
      } catch (e) {
        appStore.showMessage("Failed to delete model", "error");
      } finally {
        deletingModelLoading.value = false;
      }
    }
    onMounted(() => {
      fetchProviders();
      fetchProviderOptions();
    });
    return (_ctx, _cache) => {
      return openBlock(), createElementBlock("div", null, [
        createBaseVNode("div", _hoisted_1, [
          createVNode(VIcon, {
            color: "primary",
            class: "mr-2"
          }, {
            default: withCtx(() => [..._cache[18] || (_cache[18] = [
              createTextVNode("mdi-robot", -1)
            ])]),
            _: 1
          }),
          _cache[19] || (_cache[19] = createBaseVNode("h2", { class: "text-h5 font-weight-bold" }, "Model Settings", -1))
        ]),
        _cache[44] || (_cache[44] = createBaseVNode("p", { class: "text-body-2 text-medium-emphasis mb-4" }, " Manage model providers and configure default model settings. ", -1)),
        createVNode(VCard, { class: "mb-4" }, {
          default: withCtx(() => [
            createVNode(VCardTitle, { class: "d-flex align-center" }, {
              default: withCtx(() => [
                createVNode(VIcon, { start: "" }, {
                  default: withCtx(() => [..._cache[20] || (_cache[20] = [
                    createTextVNode("mdi-server", -1)
                  ])]),
                  _: 1
                }),
                _cache[21] || (_cache[21] = createTextVNode(" Model Providers ", -1))
              ]),
              _: 1
            }),
            createVNode(VCardText, null, {
              default: withCtx(() => [
                providersLoading.value ? (openBlock(), createBlock(VList, {
                  key: 0,
                  class: "pa-4 text-center"
                }, {
                  default: withCtx(() => [
                    createVNode(VProgressCircular, {
                      indeterminate: "",
                      size: "24",
                      width: "2",
                      color: "primary"
                    })
                  ]),
                  _: 1
                })) : (openBlock(), createBlock(VList, {
                  key: 1,
                  density: "compact"
                }, {
                  default: withCtx(() => [
                    (openBlock(true), createElementBlock(Fragment, null, renderList(providers.value, (provider) => {
                      return openBlock(), createElementBlock(Fragment, {
                        key: provider.id
                      }, [
                        createVNode(VListItem, {
                          rounded: "lg",
                          class: "mb-1",
                          onClick: ($event) => toggleExpand(provider.id)
                        }, {
                          prepend: withCtx(() => [
                            createVNode(VIcon, {
                              color: provider.enabled ? "success" : "medium-emphasis"
                            }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(provider.enabled ? "mdi-server" : "mdi-server-off"), 1)
                              ]),
                              _: 2
                            }, 1032, ["color"])
                          ]),
                          append: withCtx(() => [
                            createVNode(VChip, {
                              color: provider.enabled ? "success" : "medium-emphasis",
                              size: "x-small",
                              variant: "tonal",
                              class: "mr-1"
                            }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(provider.enabled ? "On" : "Off"), 1)
                              ]),
                              _: 2
                            }, 1032, ["color"]),
                            createVNode(VIcon, {
                              size: "18",
                              color: "medium-emphasis",
                              class: "mr-1"
                            }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(isExpanded(provider.id) ? "mdi-chevron-up" : "mdi-chevron-down"), 1)
                              ]),
                              _: 2
                            }, 1024),
                            createVNode(VBtn, {
                              icon: "mdi-pencil",
                              variant: "text",
                              size: "small",
                              onClick: withModifiers(($event) => openEditDialog(provider), ["stop"])
                            }, null, 8, ["onClick"]),
                            createVNode(VBtn, {
                              icon: "mdi-delete",
                              variant: "text",
                              size: "small",
                              color: "error",
                              onClick: withModifiers(($event) => confirmDelete(provider), ["stop"])
                            }, null, 8, ["onClick"])
                          ]),
                          default: withCtx(() => [
                            createVNode(VListItemTitle, { class: "font-weight-medium" }, {
                              default: withCtx(() => [
                                createTextVNode(toDisplayString(provider.name), 1)
                              ]),
                              _: 2
                            }, 1024),
                            createVNode(VListItemSubtitle, { class: "text-caption" }, {
                              default: withCtx(() => [
                                createBaseVNode("span", _hoisted_2, toDisplayString(provider.id), 1),
                                provider.models.length ? (openBlock(), createElementBlock("span", _hoisted_3, " · " + toDisplayString(provider.models.length) + " models", 1)) : createCommentVNode("", true)
                              ]),
                              _: 2
                            }, 1024)
                          ]),
                          _: 2
                        }, 1032, ["onClick"]),
                        createVNode(VExpandTransition, null, {
                          default: withCtx(() => [
                            withDirectives(createBaseVNode("div", _hoisted_4, [
                              createVNode(VDivider, { class: "mb-1" }),
                              _cache[25] || (_cache[25] = createBaseVNode("div", { class: "text-caption text-medium-emphasis pa-2 font-weight-bold" }, "MODELS", -1)),
                              provider.models.length === 0 ? (openBlock(), createElementBlock("div", _hoisted_5, [..._cache[22] || (_cache[22] = [
                                createBaseVNode("span", { class: "text-caption text-medium-emphasis" }, "No models", -1)
                              ])])) : createCommentVNode("", true),
                              createVNode(VList, {
                                density: "compact",
                                nav: "",
                                class: "px-1"
                              }, {
                                default: withCtx(() => [
                                  (openBlock(true), createElementBlock(Fragment, null, renderList(provider.models, (model) => {
                                    return openBlock(), createBlock(VListItem, {
                                      key: model.id,
                                      rounded: "lg",
                                      class: "mb-1"
                                    }, {
                                      prepend: withCtx(() => [
                                        createVNode(VIcon, {
                                          size: "16",
                                          color: "medium-emphasis"
                                        }, {
                                          default: withCtx(() => [..._cache[23] || (_cache[23] = [
                                            createTextVNode("mdi-cube-outline", -1)
                                          ])]),
                                          _: 1
                                        })
                                      ]),
                                      append: withCtx(() => [
                                        createVNode(VBtn, {
                                          icon: "mdi-pencil",
                                          variant: "text",
                                          size: "x-small",
                                          onClick: withModifiers(($event) => openEditModelDialog(provider.id, model), ["stop"])
                                        }, null, 8, ["onClick"]),
                                        createVNode(VBtn, {
                                          icon: "mdi-delete",
                                          variant: "text",
                                          size: "x-small",
                                          color: "error",
                                          onClick: withModifiers(($event) => confirmDeleteModel(provider.id, model), ["stop"])
                                        }, null, 8, ["onClick"])
                                      ]),
                                      default: withCtx(() => [
                                        createVNode(VListItemTitle, { class: "text-body-2" }, {
                                          default: withCtx(() => [
                                            createTextVNode(toDisplayString(model.name), 1)
                                          ]),
                                          _: 2
                                        }, 1024),
                                        createVNode(VListItemSubtitle, { class: "text-caption text-medium-emphasis" }, {
                                          default: withCtx(() => [
                                            createTextVNode(toDisplayString(model.id), 1)
                                          ]),
                                          _: 2
                                        }, 1024)
                                      ]),
                                      _: 2
                                    }, 1024);
                                  }), 128))
                                ]),
                                _: 2
                              }, 1024),
                              createBaseVNode("div", _hoisted_6, [
                                createVNode(VBtn, {
                                  variant: "tonal",
                                  size: "x-small",
                                  "prepend-icon": "mdi-plus",
                                  onClick: withModifiers(($event) => openAddModelDialog(provider.id), ["stop"])
                                }, {
                                  default: withCtx(() => [..._cache[24] || (_cache[24] = [
                                    createTextVNode(" Add Model ", -1)
                                  ])]),
                                  _: 1
                                }, 8, ["onClick"])
                              ])
                            ], 512), [
                              [vShow, isExpanded(provider.id)]
                            ])
                          ]),
                          _: 2
                        }, 1024)
                      ], 64);
                    }), 128)),
                    providers.value.length === 0 ? (openBlock(), createBlock(VListItem, { key: 0 }, {
                      default: withCtx(() => [
                        createVNode(VListItemTitle, { class: "text-medium-emphasis" }, {
                          default: withCtx(() => [..._cache[26] || (_cache[26] = [
                            createTextVNode("No providers configured", -1)
                          ])]),
                          _: 1
                        })
                      ]),
                      _: 1
                    })) : createCommentVNode("", true)
                  ]),
                  _: 1
                })),
                createVNode(VBtn, {
                  color: "primary",
                  "prepend-icon": "mdi-plus",
                  class: "mt-2",
                  onClick: openAddDialog
                }, {
                  default: withCtx(() => [..._cache[27] || (_cache[27] = [
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
                  default: withCtx(() => [..._cache[28] || (_cache[28] = [
                    createTextVNode("mdi-star", -1)
                  ])]),
                  _: 1
                }),
                _cache[29] || (_cache[29] = createTextVNode(" Default Settings ", -1))
              ]),
              _: 1
            }),
            createVNode(VCardText, null, {
              default: withCtx(() => [
                createVNode(VSelect, {
                  modelValue: unref(settings).settings.default_provider,
                  "onUpdate:modelValue": _cache[0] || (_cache[0] = ($event) => unref(settings).settings.default_provider = $event),
                  label: "Default Provider",
                  items: providerOptions.value,
                  variant: "outlined",
                  class: "mb-4",
                  disabled: !providerOptions.value.length
                }, null, 8, ["modelValue", "items", "disabled"]),
                createVNode(VSelect, {
                  modelValue: unref(settings).settings.default_model,
                  "onUpdate:modelValue": _cache[1] || (_cache[1] = ($event) => unref(settings).settings.default_model = $event),
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
        createBaseVNode("div", _hoisted_7, [
          createVNode(VBtn, {
            color: "primary",
            size: "large",
            loading: updatingDefault.value,
            onClick: handleUpdateDefaultModel
          }, {
            default: withCtx(() => [
              createVNode(VIcon, { start: "" }, {
                default: withCtx(() => [..._cache[30] || (_cache[30] = [
                  createTextVNode("mdi-content-save", -1)
                ])]),
                _: 1
              }),
              _cache[31] || (_cache[31] = createTextVNode(" Update Default Model ", -1))
            ]),
            _: 1
          }, 8, ["loading"])
        ]),
        createVNode(VDialog, {
          modelValue: dialogOpen.value,
          "onUpdate:modelValue": _cache[9] || (_cache[9] = ($event) => dialogOpen.value = $event),
          "max-width": "500"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(isEditing.value ? "Edit Provider" : "Add Provider"), 1)
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        isEditing.value ? (openBlock(), createBlock(VCol, {
                          key: 0,
                          cols: "12"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.name,
                              "onUpdate:modelValue": _cache[2] || (_cache[2] = ($event) => form.value.name = $event),
                              label: "Name",
                              variant: "outlined"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        })) : createCommentVNode("", true),
                        !isEditing.value ? (openBlock(), createBlock(VCol, {
                          key: 1,
                          cols: "12"
                        }, {
                          default: withCtx(() => [
                            createVNode(VAutocomplete, {
                              modelValue: form.value.id,
                              "onUpdate:modelValue": [
                                _cache[3] || (_cache[3] = ($event) => form.value.id = $event),
                                onProviderSelected
                              ],
                              label: "Provider",
                              variant: "outlined",
                              items: providerOptionItems.value,
                              "item-title": "title",
                              "item-value": "id",
                              loading: optionsLoading.value,
                              rules: [(v) => !!v || "Required"]
                            }, {
                              item: withCtx(({ props, item }) => [
                                createVNode(VListItem, mergeProps(props, {
                                  subtitle: item.raw.description,
                                  "prepend-icon": "mdi-server"
                                }), {
                                  title: withCtx(() => [
                                    createBaseVNode("div", _hoisted_8, [
                                      createTextVNode(toDisplayString(item.raw.title) + " ", 1),
                                      item.raw.category ? (openBlock(), createBlock(VChip, {
                                        key: 0,
                                        size: "x-small",
                                        variant: "tonal",
                                        class: "ml-2"
                                      }, {
                                        default: withCtx(() => [
                                          createTextVNode(toDisplayString(item.raw.category), 1)
                                        ]),
                                        _: 2
                                      }, 1024)) : createCommentVNode("", true)
                                    ])
                                  ]),
                                  _: 2
                                }, 1040, ["subtitle"])
                              ]),
                              _: 1
                            }, 8, ["modelValue", "items", "loading", "rules"])
                          ]),
                          _: 1
                        })) : createCommentVNode("", true),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.api_base,
                              "onUpdate:modelValue": _cache[4] || (_cache[4] = ($event) => form.value.api_base = $event),
                              label: "API Base URL",
                              variant: "outlined",
                              placeholder: "https://api.example.com/v1"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        }),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: form.value.api_key,
                              "onUpdate:modelValue": _cache[5] || (_cache[5] = ($event) => form.value.api_key = $event),
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
                            createVNode(VSwitch, {
                              modelValue: form.value.enabled,
                              "onUpdate:modelValue": _cache[7] || (_cache[7] = ($event) => form.value.enabled = $event),
                              label: "Enabled",
                              color: "success",
                              density: "compact",
                              "hide-details": ""
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
                      onClick: _cache[8] || (_cache[8] = ($event) => dialogOpen.value = false)
                    }, {
                      default: withCtx(() => [..._cache[32] || (_cache[32] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      loading: savingProvider.value,
                      onClick: handleSaveProvider
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(isEditing.value ? "Update" : "Add"), 1)
                      ]),
                      _: 1
                    }, 8, ["loading"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: deleteConfirmOpen.value,
          "onUpdate:modelValue": _cache[11] || (_cache[11] = ($event) => deleteConfirmOpen.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[33] || (_cache[33] = [
                    createTextVNode("Delete Provider", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      _cache[34] || (_cache[34] = createTextVNode(" Are you sure you want to delete ", -1)),
                      createBaseVNode("strong", null, toDisplayString((_a = deletingProvider.value) == null ? void 0 : _a.name), 1),
                      _cache[35] || (_cache[35] = createTextVNode("? ", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[10] || (_cache[10] = ($event) => deleteConfirmOpen.value = false)
                    }, {
                      default: withCtx(() => [..._cache[36] || (_cache[36] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      loading: deletingProviderLoading.value,
                      onClick: handleDeleteProvider
                    }, {
                      default: withCtx(() => [..._cache[37] || (_cache[37] = [
                        createTextVNode("Delete", -1)
                      ])]),
                      _: 1
                    }, 8, ["loading"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: modelDialogOpen.value,
          "onUpdate:modelValue": _cache[15] || (_cache[15] = ($event) => modelDialogOpen.value = $event),
          "max-width": "500"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [
                    createTextVNode(toDisplayString(isEditingModel.value ? "Edit Model" : "Add Model"), 1)
                  ]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => [
                    createVNode(VRow, { dense: "" }, {
                      default: withCtx(() => [
                        !isEditingModel.value ? (openBlock(), createBlock(VCol, {
                          key: 0,
                          cols: "12"
                        }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: modelForm.value.id,
                              "onUpdate:modelValue": _cache[12] || (_cache[12] = ($event) => modelForm.value.id = $event),
                              label: "Model ID",
                              variant: "outlined",
                              placeholder: "deepseek/deepseek-v4-pro"
                            }, null, 8, ["modelValue"])
                          ]),
                          _: 1
                        })) : createCommentVNode("", true),
                        createVNode(VCol, { cols: "12" }, {
                          default: withCtx(() => [
                            createVNode(VTextField, {
                              modelValue: modelForm.value.name,
                              "onUpdate:modelValue": _cache[13] || (_cache[13] = ($event) => modelForm.value.name = $event),
                              label: "Name",
                              variant: "outlined"
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
                      onClick: _cache[14] || (_cache[14] = ($event) => modelDialogOpen.value = false)
                    }, {
                      default: withCtx(() => [..._cache[38] || (_cache[38] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "primary",
                      loading: savingModel.value,
                      onClick: handleSaveModel
                    }, {
                      default: withCtx(() => [
                        createTextVNode(toDisplayString(isEditingModel.value ? "Update" : "Add"), 1)
                      ]),
                      _: 1
                    }, 8, ["loading"])
                  ]),
                  _: 1
                })
              ]),
              _: 1
            })
          ]),
          _: 1
        }, 8, ["modelValue"]),
        createVNode(VDialog, {
          modelValue: deleteModelConfirmOpen.value,
          "onUpdate:modelValue": _cache[17] || (_cache[17] = ($event) => deleteModelConfirmOpen.value = $event),
          "max-width": "400"
        }, {
          default: withCtx(() => [
            createVNode(VCard, null, {
              default: withCtx(() => [
                createVNode(VCardTitle, null, {
                  default: withCtx(() => [..._cache[39] || (_cache[39] = [
                    createTextVNode("Delete Model", -1)
                  ])]),
                  _: 1
                }),
                createVNode(VCardText, null, {
                  default: withCtx(() => {
                    var _a;
                    return [
                      _cache[40] || (_cache[40] = createTextVNode(" Are you sure you want to delete ", -1)),
                      createBaseVNode("strong", null, toDisplayString((_a = deletingModel.value) == null ? void 0 : _a.name), 1),
                      _cache[41] || (_cache[41] = createTextVNode("? ", -1))
                    ];
                  }),
                  _: 1
                }),
                createVNode(VCardActions, null, {
                  default: withCtx(() => [
                    createVNode(VSpacer),
                    createVNode(VBtn, {
                      variant: "text",
                      onClick: _cache[16] || (_cache[16] = ($event) => deleteModelConfirmOpen.value = false)
                    }, {
                      default: withCtx(() => [..._cache[42] || (_cache[42] = [
                        createTextVNode("Cancel", -1)
                      ])]),
                      _: 1
                    }),
                    createVNode(VBtn, {
                      color: "error",
                      loading: deletingModelLoading.value,
                      onClick: handleDeleteModel
                    }, {
                      default: withCtx(() => [..._cache[43] || (_cache[43] = [
                        createTextVNode("Delete", -1)
                      ])]),
                      _: 1
                    }, 8, ["loading"])
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
